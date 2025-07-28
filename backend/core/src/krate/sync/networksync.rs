use core::sync;
use std::marker::PhantomData;

use crate::krate::sync::controller::{SYNC_DATA_PATH, SYNC_FETCH_TID_PATH};
use crate::krate::sync::dto::{
    SyncDataArg, SyncFetchTIDArg, SyncFetchTIDReq, SyncPageInfo, SyncShakeArg, SyncShakeDto,
};
use crate::model::KOtidSupport;
use crate::sync_cmds_json_to_st;
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
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_SHAKE_PATH
            ))
            .json(&SyncShakeReq {
                table_type: T::get_otid_enum(),
                dto: SyncShakeDto {
                    instance_id: self.instance_id.clone(),
                    db_version: DB_VERSION.to_string(),
                },
            })
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

        let rsp = client
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_FETCH_TID_PATH
            ))
            .json(&SyncFetchTIDReq {
                table_type: T::get_otid_enum(),
                dto: fetch_data.dto,
            })
            .send()
            .await?
            .json::<SyncFetchTIDRsp>()
            .await?;

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

    pub(crate) async fn sync_data_tx<T, F, Fut>(
        &self,
        endpoint: &SyncEndpoint,
        sync_info: &SyncPageInfo<T>,
        before_send_operations: F,
    ) -> AResult<SyncDataArg<T>>
    where
        Fut: Future<Output = EResult>,
        F: Fn(&SyncDataArg<T>) -> Fut,
        T: KOtidSupport,
    {
        let dto: SyncDataArg<T> = self.mapper.sync_fetch_operations(sync_info).await?;
        before_send_operations(&dto).await?;
        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_DATA_PATH
            ))
            .json(&dto)
            .send()
            .await?
            .json::<SyncDataArg<serde_json::Value>>()
            .await?;
        let cmds = sync_cmds_json_to_st!(T, rsp.cmds);

        Ok(SyncDataArg {
            cmds: cmds?,
            max_tid: rsp.max_tid,
            nomore: rsp.nomore,
        })
    }

    pub(crate) async fn sync_one_otid_table1<T: KOtidSupport>(
        &self,
        endpoint: &SyncEndpoint,
    ) -> EResult {
        // TODO: why this function is not right.
        /*        async fn empty_ok_fut<'a, T>(_: &'a SyncDataArg<T>) -> EResult {
            Ok(())
        } */
        let empty_ok_fut = |_: &SyncDataArg<T>| async move { Ok(()) };
        self.sync_one_otid_table(PhantomData::<T>, endpoint, empty_ok_fut, empty_ok_fut)
            .await?;
        Ok(())
    }

    pub(crate) async fn sync_one_otid_table<T, F, Fut>(
        &self,
        _: PhantomData<T>,
        endpoint: &SyncEndpoint,
        before_send_operations: F,
        before_merge_operations: F,
    ) -> EResult
    where
        T: KOtidSupport,
        Fut: Future<Output = EResult>,
        F: Fn(&SyncDataArg<T>) -> Fut + Clone,
    {
        use crate::krate::sync::dto::*;
        use crate::krate::sync::mapper::SyncMapper as _;
        use chin_sql::time_type::TID;

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

        let initial_sync_info = SyncInfo {
            instance_id: shake_rsp.instance_id.clone(),
            start_ex: sync_time,
            end_in: TID::default(),
            table_type: std::marker::PhantomData,
        };

        let mut start_ex = initial_sync_info.start_ex;
        let mut hist = false;
        loop {
            let result: SyncFetchTIDRsp = self
                .sync_fetch_tids_tx::<T>(
                    endpoint,
                    OtidWithGer {
                        dto: SyncFetchTIDDto {
                            page: SyncFetchDataPageInfo::StartEnd {
                                start_ex,
                                end_in: initial_sync_info.end_in,
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
            log::info!("max sync time: {sync_time}({result_len})");
            self.sync_merge_tids(result, hist, initial_sync_info.clone())
                .await?;
            if result_len < page_size {
                if hist {
                    break;
                } else {
                    hist = true;
                    start_ex = initial_sync_info.start_ex;
                }
            }
        }

        let mut start_ex = initial_sync_info.start_ex;
        loop {
            let sync_info = SyncPageInfo {
                sync_info: initial_sync_info.clone(),
                page_size,
                start_ex,
            };
            let rsp: SyncDataArg<T> = self
                .sync_data_tx(endpoint, &sync_info, &before_send_operations)
                .await?;
            let nomore = rsp.nomore;
            before_merge_operations(&rsp).await?;
            start_ex = rsp.max_tid;
            self.sync_merge_operations(rsp).await?;
            if nomore {
                break;
            }
        }

        Ok(())
    }
}
