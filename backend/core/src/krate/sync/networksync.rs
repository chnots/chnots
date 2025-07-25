use crate::krate::sync::controller::SYNC_FETCH_TID_PATH;
use crate::krate::sync::dto::{FetchTIDReq, SyncInfo};
use crate::{
    app::ShareAppState,
    krate::sync::{
        controller::SYNC_SHAKE_PATH,
        dto::{
            SyncFetchTIDReq, SyncFetchTIDRsp, SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum,
            SyncTableEnum,
        },
        mapper::SyncMapper,
    },
    magics::DB_VERSION,
};
use chin_tools::{AResult, EResult, SharedStr};

use super::po::SyncEndpoint;

impl ShareAppState {
    pub(crate) async fn sync_shake_rx(&self, req: SyncShakeReq) -> AResult<SyncShakeRsp> {
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
                req.table_name.to_table_name().try_into()?,
                req.instance_id.to_string().try_into()?,
            )
            .await?;

        Ok(SyncShakeRsp {
            instance_id: instace_id,
            data: SyncShakeRspEnum::BeginSync { sync_time },
        })
    }

    pub(crate) async fn sync_shake_tx(
        &self,
        endpoint: &SyncEndpoint,
        ste: SyncTableEnum,
    ) -> AResult<SyncShakeRsp> {
        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_SHAKE_PATH
            ))
            .json(&SyncShakeReq {
                instance_id: self.instance_id.clone(),
                db_version: DB_VERSION.to_string(),
                table_name: ste,
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
                ste.to_table_name().try_into()?,
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

    pub(crate) async fn sync_fetch_tids_tx(
        &self,
        endpoint: &SyncEndpoint,
        instance_id: SharedStr,
        ste: SyncTableEnum,
        fetch_data: FetchTIDReq,
        hist: bool,
    ) -> AResult<SyncFetchTIDRsp> {
        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_FETCH_TID_PATH
            ))
            .json(&SyncFetchTIDReq {
                sync_info: SyncInfo {
                    instance_id,
                    start_ex: fetch_data.start_ex,
                    end_in: fetch_data.end_in,
                    table: ste,
                },
                page_size: fetch_data.page_size,
                hist,
            })
            .send()
            .await?
            .json::<SyncFetchTIDRsp>()
            .await?;

        Ok(rsp)
    }

    pub(crate) async fn sync_fetch_tids_rx(
        &self,
        req: SyncFetchTIDReq,
    ) -> AResult<SyncFetchTIDRsp> {
        self.mapper.sync_dump_tids(req).await
    }

    // pub(crate) async fn sync_fetch_data_rx(
    //     &self,
    //     req: SyncFetchTIDReq,
    // ) -> AResult<SyncFetchTIDRsp> {
    //     macro_rules! get {
    //         ($st:ty) => {
    //             let c: Vec<$st> = self
    //                 .mapper
    //                 .sync_get_records(
    //                     req.fetch_data,
    //                     |e| {
    //                         let c: AResult<$st> = match e {
    //                             crate::mapper::MapperRowType::KDb(kdb_row) => kdb_row.try_into(),
    //                         };
    //                         c
    //                     },
    //                 )
    //                 .await?;

    //             c.into_iter()
    //                 .map(|e| serde_json::to_value(e))
    //                 .collect::<Result<Vec<serde_json::Value>, serde_json::Error>>()?
    //         };
    //     }

    //     let records: Vec<Value> = match req.table_name {
    //         super::dto::SyncTableEnum::ChnotRecord => {
    //             get! {ChnotRecord}
    //         }
    //         super::dto::SyncTableEnum::ChnotMetadata => {
    //             get! {ChnotMetadata}
    //         }
    //         super::dto::SyncTableEnum::ChnotKindRel => {
    //             get! {ChnotKindRel}
    //         }
    //         super::dto::SyncTableEnum::ChnotTag => {
    //             get! {ChnotTag}
    //         }
    //         super::dto::SyncTableEnum::LLMChatBot => {
    //             get! {LLMChatBot}
    //         }
    //         super::dto::SyncTableEnum::LLMChatRecord => {
    //             get! {LLMChatRecord}
    //         }
    //         super::dto::SyncTableEnum::LLMChatTemplate => {
    //             get! {LLMChatTemplate}
    //         }
    //         super::dto::SyncTableEnum::LLMChatSession => {
    //             get! {LLMChatSession}
    //         }
    //         super::dto::SyncTableEnum::KKV => {
    //             get! {KKV}
    //         }
    //         super::dto::SyncTableEnum::KTabMeta => {
    //             get! {KTabMeta}
    //         }
    //         super::dto::SyncTableEnum::KTabCellDate => {
    //             get! {KTabCellDate}
    //         }
    //         super::dto::SyncTableEnum::KTabCellDecimal => {
    //             get! {KTabCellDecimal}
    //         }
    //         super::dto::SyncTableEnum::KTabCellText => {
    //             get! {KTabCellText}
    //         }
    //         super::dto::SyncTableEnum::KFileMeta => {
    //             get! {KFileMeta}
    //         }
    //     };
    //     Ok(SyncFetchTIDRsp { hists: records })
    // }
}

async fn sync_tids(
    app: ShareAppState,
    endpoint: SyncEndpoint,
    hist: bool,
    table: SyncTableEnum,
) -> EResult {
    use crate::krate::sync::dto::*;
    use crate::krate::sync::mapper::SyncMapper as _;
    use chin_sql::time_type::TID;

    let shake_rsp = app
        .sync_shake_tx(&endpoint, SyncTableEnum::ChnotRecord)
        .await?;
    let mut sync_time = match shake_rsp.data {
        SyncShakeRspEnum::NotSameVersion(nsv) => {
            anyhow::bail!("not same version {}", nsv);
        }
        SyncShakeRspEnum::SameClient => {
            anyhow::bail!("Same Client");
        }
        SyncShakeRspEnum::BeginSync { sync_time } => sync_time,
    };

    let sync_info = SyncInfo {
        instance_id: shake_rsp.instance_id.clone(),
        start_ex: sync_time,
        end_in: TID::default(),
        table,
    };

    let page_size = 500;
    loop {
        let result: SyncFetchTIDRsp = app
            .sync_fetch_tids_tx(
                &endpoint,
                shake_rsp.instance_id.clone(),
                SyncTableEnum::ChnotRecord,
                FetchTIDReq {
                    start_ex: sync_time,
                    end_in: TID::default(),
                    page_size,
                },
                hist,
            )
            .await?;
        let result_len = result.data.len();
        let c_sync_time = result.data.iter().max();
        if let Some(c_sync_time) = c_sync_time {
            sync_time = *c_sync_time;
        }
        log::info!("max sync time: {sync_time}({result_len})");
        app.sync_merge_tids(result, hist, sync_info.clone()).await?;
        if result_len < page_size {
            break;
        }
    }

    Ok(())
}

async fn parse_tids(app: ShareAppState) -> AResult<()> {}

#[macro_export]
macro_rules! sync_one {
    ($app:ident, $st:ident, $endpoint:ident, $hist:expr, $before_actions:expr) => {};
    ($app:ident, $st:ident, $endpoint:ident, $hist:expr) => {
        sync_one!($app, $st, $endpoint, $hist, async |_, _, _| {
            Ok::<(), anyhow::Error>(())
        })
    };
}
