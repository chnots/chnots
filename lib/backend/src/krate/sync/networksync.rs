use std::marker::PhantomData;
use std::time::Duration;

use crate::krate::sync::controller::{SYNC_DATA_PATH, SYNC_FETCH_TID_PATH, SYNC_INSERT_SYNC_LOG};
use crate::krate::sync::dto::{
    SyncDataArg, SyncDataDto, SyncDataOperation, SyncDataReqRsp, SyncFetchTIDArg, SyncFetchTIDReq,
    SyncPageInfo, SyncShakeArg, SyncShakeDto,
};
use crate::krate::sync::po::SyncLogTransient;
use crate::model::KOtidSupport;
use crate::sync_cmds_st_to_json;
use crate::{
    app::ShareAppState,
    krate::sync::{
        controller::SYNC_SHAKE_PATH,
        dto::{SyncFetchTIDRsp, SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum},
        mapper::SyncMapper,
    },
    magics::DB_VERSION,
};
use chin_tools::{AResult, EResult};
use log::info;

use super::po::SyncEndpoint;

impl ShareAppState {
    pub(crate) async fn sync_shake_rx<T: KOtidSupport>(
        &self,
        req: SyncShakeArg<T>,
    ) -> AResult<SyncShakeRsp> {
        let instace_id = self.instance_id.clone();

        if req.db_version.as_str() != DB_VERSION {
            return Ok(SyncShakeRsp {
                instance_id: instace_id,
                data: SyncShakeRspEnum::NotSameVersion(DB_VERSION.to_owned()),
            });
        }

        if req.instance_id == instace_id {
            return Ok(SyncShakeRsp {
                instance_id: instace_id,
                data: SyncShakeRspEnum::SameClient,
            });
        }

        let sync_time = self
            .mapper
            .sync_get_sync_time(
                T::table_name(false).try_into()?,
                req.instance_id.to_string().try_into()?,
            )
            .await?;

        Ok(SyncShakeRsp {
            instance_id: instace_id,
            data: SyncShakeRspEnum::BeginSync { sync_time },
        })
    }

    pub(crate) async fn sync_shake_tx<T: KOtidSupport>(
        &self,
        endpoint: &SyncEndpoint,
    ) -> AResult<SyncShakeRsp> {
        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(endpoint.to_url(SYNC_SHAKE_PATH))
            .json(&SyncShakeReq {
                table_type: T::get_otid_enum(),
                dto: SyncShakeDto {
                    instance_id: self.instance_id.clone(),
                    db_version: DB_VERSION.to_string(),
                },
            })
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .json::<SyncShakeRsp>()
            .await?;

        let rtime = match rsp.data {
            SyncShakeRspEnum::NotSameVersion(nsv) => {
                return Ok(SyncShakeRsp {
                    instance_id: rsp.instance_id,
                    data: SyncShakeRspEnum::NotSameVersion(nsv),
                });
            }
            SyncShakeRspEnum::BeginSync { sync_time } => sync_time,
            SyncShakeRspEnum::SameClient => {
                return Ok(SyncShakeRsp {
                    instance_id: rsp.instance_id,
                    data: SyncShakeRspEnum::SameClient,
                });
            }
        };

        let stime = self
            .mapper
            .sync_get_sync_time(
                T::table_name(false).try_into()?,
                rsp.instance_id.to_string().try_into()?,
            )
            .await?;

        let sync_tid = if stime > rtime { rtime } else { stime };

        Ok(SyncShakeRsp {
            instance_id: rsp.instance_id,
            data: SyncShakeRspEnum::BeginSync {
                sync_time: sync_tid,
            },
        })
    }

    pub(crate) async fn sync_fetch_tids_tx<T: KOtidSupport>(
        &self,
        endpoint: &SyncEndpoint,
        fetch_data: SyncFetchTIDArg<T>,
    ) -> AResult<SyncFetchTIDRsp> {
        let client = reqwest::Client::builder().build()?;
        let page = fetch_data.dto.page.clone();

        let rsp = client
            .post(endpoint.to_url(SYNC_FETCH_TID_PATH))
            .json(&SyncFetchTIDReq {
                table_type: T::get_otid_enum(),
                dto: fetch_data.dto,
            })
            .send()
            .await?
            .json::<SyncFetchTIDRsp>()
            .await?;

        info!(
            "fetch tid req, {}({:?}) rsp size: {}",
            T::table_name(false),
            page,
            rsp.data.len()
        );

        Ok(rsp)
    }

    pub(crate) async fn sync_fetch_tids_rx<T: KOtidSupport>(
        &self,
        req: SyncFetchTIDArg<T>,
    ) -> AResult<SyncFetchTIDRsp> {
        self.mapper.sync_fetch_tids(req).await
    }

    pub(crate) async fn sync_data_rx<T: KOtidSupport>(
        &self,
        req: SyncDataArg<T>,
    ) -> AResult<SyncDataArg<T>> {
        self.mapper.sync_merge_operations(req).await
    }

    async fn sync_insert_sync_log_tx(
        &self,
        endpoint: &SyncEndpoint,
        log: &SyncLogTransient,
    ) -> EResult {
        let client = reqwest::Client::builder().build()?;

        client
            .post(endpoint.to_url(SYNC_INSERT_SYNC_LOG))
            .json(&log)
            .send()
            .await?;

        Ok(())
    }

    pub(crate) async fn sync_data_tx<T, W>(
        &self,
        endpoint: &SyncEndpoint,
        sync_info: &SyncPageInfo<T>,
        otid_related_worker: &W,
    ) -> AResult<SyncDataArg<T>>
    where
        W: OtidRelatedWorker<T>,
        T: KOtidSupport,
    {
        let dto: SyncDataArg<T> = self.mapper.sync_fetch_operations(sync_info).await?;
        otid_related_worker.before_send(endpoint, &dto).await?;

        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(endpoint.to_url(SYNC_DATA_PATH))
            .json(&SyncDataReqRsp {
                table_type: T::get_otid_enum(),
                dto: SyncDataDto {
                    cmds: sync_cmds_st_to_json!(dto.cmds),
                    max_tid: dto.max_tid,
                    nomore: dto.nomore,
                },
            })
            .send()
            .await?
            .json::<SyncDataReqRsp>()
            .await?;
        let res: Result<Vec<SyncDataOperation<T>>, serde_json::Error> = rsp
            .dto
            .cmds
            .into_iter()
            .map(|s| {
                let c = match s {
                    crate::krate::sync::dto::SyncDataOperation::Omit(tid) => {
                        SyncDataOperation::Omit(tid)
                    }
                    crate::krate::sync::dto::SyncDataOperation::Pull { tid, hist } => {
                        SyncDataOperation::Pull { tid, hist }
                    }
                    crate::krate::sync::dto::SyncDataOperation::Push { data, hist } => {
                        SyncDataOperation::Push {
                            data: {
                                let c: T = serde_json::from_str(&data)?;
                                c
                            },
                            hist,
                        }
                    }
                };
                Ok(c)
            })
            .collect();

        Ok(SyncDataArg {
            cmds: res?,
            max_tid: dto.max_tid,
            nomore: dto.nomore,
        })
    }

    pub(crate) async fn sync_one_otid_table1<T: KOtidSupport>(
        &self,
        endpoint: &SyncEndpoint,
    ) -> EResult {
        self.sync_one_otid_table(PhantomData::<T>, endpoint, &SimpleOtidRelatedWorker {})
            .await?;
        Ok(())
    }

    pub(crate) async fn sync_one_otid_table<T, W>(
        &self,
        _: PhantomData<T>,
        endpoint: &SyncEndpoint,
        otid_related_worker: &W,
    ) -> EResult
    where
        T: KOtidSupport,
        W: OtidRelatedWorker<T>,
    {
        use crate::krate::sync::dto::*;
        use crate::krate::sync::mapper::SyncMapper as _;
        use chin_sql::time_type::TID;

        info!("sync one otid table: {:?}", T::get_otid_enum());
        let page_size = 100;
        let shake_rsp = self.sync_shake_tx::<T>(endpoint).await?;
        let sync_time = match shake_rsp.data {
            SyncShakeRspEnum::NotSameVersion(nsv) => {
                anyhow::bail!("not same version {}", nsv);
            }
            SyncShakeRspEnum::SameClient => {
                anyhow::bail!("Same Client");
            }
            SyncShakeRspEnum::BeginSync { sync_time } => sync_time,
        };

        info!(
            "remote sync start time: {}|{}",
            endpoint.ip,
            sync_time.as_utc()
        );
        let sync_info = SyncInfo {
            instance_id: shake_rsp.instance_id.clone(),
            start_ex: sync_time,
            end_in: TID::default(),
            table_type: std::marker::PhantomData,
            pantient: true,
        };

        self.sync_create_tmp_table(&sync_info).await?;

        let mut start_ex = sync_info.start_ex;
        let mut hist = false;
        loop {
            info!(
                "sync page info: {:?}/{}({})",
                T::get_otid_enum(),
                start_ex,
                start_ex.as_utc()
            );
            let result: SyncFetchTIDRsp = self
                .sync_fetch_tids_tx::<T>(
                    endpoint,
                    OtidWithGer {
                        dto: SyncFetchTIDDto {
                            page: SyncFetchTIDPage::StartEnd {
                                start_ex,
                                end_in: sync_info.end_in,
                                page_size,
                            },
                            hist,
                        },
                        table_type: PhantomData,
                    },
                )
                .await?;
            let result_len = result.data.len();
            let c_sync_time = result.data.iter().max();
            if let Some(cstart_ex) = c_sync_time {
                start_ex = *cstart_ex;
            }
            self.sync_merge_tids(result, hist, sync_info.clone())
                .await?;
            if result_len < page_size {
                if hist {
                    break;
                } else {
                    hist = true;
                    start_ex = sync_info.start_ex;
                }
            }
        }

        let mut start_ex = sync_info.start_ex;
        let mut sync_step = SyncSingleStep::Omit;
        loop {
            let sync_page = SyncPageInfo {
                sync_info: sync_info.clone(),
                page_size,
                start_ex,
                sync_step,
            };

            let rsp: SyncDataArg<T> = self
                .sync_data_tx(endpoint, &sync_page, otid_related_worker)
                .await?;
            let nomore = rsp.nomore;

            otid_related_worker.before_merge(endpoint, &rsp).await?;

            start_ex = rsp.max_tid;

            self.sync_merge_operations(rsp).await?;
            if nomore {
                match sync_step {
                    SyncSingleStep::Omit => {
                        sync_step = SyncSingleStep::Data;
                        start_ex = sync_info.start_ex;
                    }
                    SyncSingleStep::Data => {
                        break;
                    }
                }
            }
        }

        // self.sync_drop_tmp_table(&sync_info).await?;

        let remote_log = SyncLogTransient {
            remote_id: self.instance_id.to_string().try_into()?,
            table_name: T::table_name(false).try_into()?,
            end_sync_in: sync_info.end_in,
            start_tid_ex: sync_info.start_ex,
            sync_finish_tid: TID::default(),
        };
        self.sync_insert_sync_log_tx(endpoint, &remote_log).await?;

        let local_log: SyncLogTransient = SyncLogTransient {
            remote_id: sync_info.instance_id.to_string().try_into()?,
            table_name: T::table_name(false).try_into()?,
            end_sync_in: sync_info.end_in,
            start_tid_ex: sync_info.start_ex,
            sync_finish_tid: TID::default(),
        };
        self.sync_insert_sync_log(local_log).await?;

        Ok(())
    }
}

pub(crate) trait OtidRelatedWorker<T: KOtidSupport> {
    async fn before_send(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<T>) -> EResult;
    async fn before_merge(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<T>) -> EResult;
}

struct SimpleOtidRelatedWorker;
impl<T: KOtidSupport> OtidRelatedWorker<T> for SimpleOtidRelatedWorker {
    #[inline]
    async fn before_send(&self, _: &SyncEndpoint, _: &SyncDataArg<T>) -> EResult {
        Ok(())
    }

    #[inline]
    async fn before_merge(&self, _: &SyncEndpoint, _: &SyncDataArg<T>) -> EResult {
        Ok(())
    }
}
