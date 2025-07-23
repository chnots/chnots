use crate::krate::sync::controller::SYNC_FETCH_DATA_PATH;
use crate::krate::sync::dto::FetchDataType;
use crate::krate::sync::mapper::SyncOperator;
use crate::{
    app::ShareAppState,
    krate::{
        chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
        kfile::KFileMeta,
        kkv::KKV,
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        sync::{
            controller::SYNC_SHAKE_PATH,
            dto::{
                SyncFetchDataReq, SyncFetchDataRsp, SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum,
                SyncTableEnum,
            },
            mapper::SyncMapper,
        },
    },
    magics::DB_VERSION,
};
use chin_tools::AResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::po::SyncEndpoint;

impl ShareAppState {
    pub(crate) async fn sync_shake_rx(&self, req: SyncShakeReq) -> AResult<SyncShakeRsp> {
        let instace_id = self.get_instance_id().await?;

        if req.db_version.as_str() != DB_VERSION {
            return Ok(SyncShakeRsp {
                instance_id: instace_id,
                data: SyncShakeRspEnum::NotSameVersion(DB_VERSION.to_owned()),
            });
        }

        let sync_time = self
            .mapper
            .get_sync_time(
                req.table_name.to_string().try_into()?,
                req.client_id.to_string().try_into()?,
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
                client_id: self.instance_id.clone(),
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
        };

        let stime = self
            .mapper
            .get_sync_time(
                ste.to_string().try_into()?,
                rsp.instance_id.clone().try_into()?,
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

    pub(crate) async fn sync_fetch_data_tx<E: DeserializeOwned + Serialize>(
        &self,
        endpoint: &SyncEndpoint,
        ste: SyncTableEnum,
        fetch_data: FetchDataType,
        hist: bool,
    ) -> AResult<SyncFetchDataRsp<E>> {
        let client = reqwest::Client::builder().build()?;

        let rsp = client
            .post(format!(
                "http://{}:{}{}",
                endpoint.ip, endpoint.port, SYNC_FETCH_DATA_PATH
            ))
            .json(&SyncFetchDataReq {
                table_name: ste,
                fetch_data,
                hist,
            })
            .send()
            .await?
            .json::<SyncFetchDataRsp<serde_json::Value>>()
            .await?;

        let records = rsp
            .records
            .into_iter()
            .map(|value| serde_json::from_value::<E>(value))
            .collect::<Result<Vec<_>, serde_json::Error>>()?;

        Ok(SyncFetchDataRsp { records })
    }

    pub(crate) async fn sync_fetch_data_rx(
        &self,
        req: SyncFetchDataReq,
    ) -> AResult<SyncFetchDataRsp<serde_json::Value>> {
        macro_rules! get {
            ($st:ty) => {
                let c: Vec<$st> = self
                    .mapper
                    .get(
                        req.fetch_data,
                        |e| {
                            let c: AResult<$st> = match e {
                                crate::mapper::MapperRowType::KDb(kdb_row) => kdb_row.try_into(),
                            };
                            c
                        },
                        req.hist,
                    )
                    .await?;

                c.into_iter()
                    .map(|e| serde_json::to_value(e))
                    .collect::<Result<Vec<serde_json::Value>, serde_json::Error>>()?
            };
        }

        let records: Vec<Value> = match req.table_name {
            super::dto::SyncTableEnum::ChnotRecord => {
                get! {ChnotRecord}
            }
            super::dto::SyncTableEnum::ChnotMetadata => {
                get! {ChnotMetadata}
            }
            super::dto::SyncTableEnum::ChnotKindRel => {
                get! {ChnotKindRel}
            }
            super::dto::SyncTableEnum::ChnotTag => {
                get! {ChnotTag}
            }
            super::dto::SyncTableEnum::LLMChatBot => {
                get! {LLMChatBot}
            }
            super::dto::SyncTableEnum::LLMChatRecord => {
                get! {LLMChatRecord}
            }
            super::dto::SyncTableEnum::LLMChatTemplate => {
                get! {LLMChatTemplate}
            }
            super::dto::SyncTableEnum::LLMChatSession => {
                get! {LLMChatSession}
            }
            super::dto::SyncTableEnum::KKV => {
                get! {KKV}
            }
            super::dto::SyncTableEnum::KTabMeta => {
                get! {KTabMeta}
            }
            super::dto::SyncTableEnum::KTabCellDate => {
                get! {KTabCellDate}
            }
            super::dto::SyncTableEnum::KTabCellDecimal => {
                get! {KTabCellDecimal}
            }
            super::dto::SyncTableEnum::KTabCellText => {
                get! {KTabCellText}
            }
            super::dto::SyncTableEnum::KFileMeta => {
                get! {KFileMeta}
            }
        };
        Ok(SyncFetchDataRsp { records })
    }
}

#[macro_export]
macro_rules! sync_one {
    ($app:ident, $st:ident, $endpoint:ident, $hist:expr, $before_actions:expr) => {
        let shake_rsp = $app
            .sync_shake_tx(&$endpoint, $crate::krate::sync::dto::SyncTableEnum::$st)
            .await?;
        let mut sync_time = match shake_rsp.data {
            $crate::krate::sync::dto::SyncShakeRspEnum::NotSameVersion(nsv) => {
                anyhow::bail!("not same version {}", nsv);
            }
            $crate::krate::sync::dto::SyncShakeRspEnum::BeginSync { sync_time } => sync_time,
        };
        let initial_start = sync_time;

        let page_size = 500;
        loop {
            let result: $crate::krate::sync::dto::SyncFetchDataRsp<$st> = $app
                .sync_fetch_data_tx(
                    &$endpoint,
                    $crate::krate::sync::dto::SyncTableEnum::$st,
                    $crate::krate::sync::dto::FetchDataType::RangePage {
                        start_ex: sync_time,
                        end_in: TID::default(),
                        page_size,
                    },
                    $hist,
                )
                .await?;
            let result_len = result.records.len();
            let c_sync_time = result
                .records
                .iter()
                .map(|rec| rec.tid)
                .max();
            if let Some(c_sync_time) = c_sync_time{
                sync_time = c_sync_time;
            }
            log::info!("max sync time: {}({})", sync_time, result_len);
            $before_actions(&$app, &$endpoint, &result.records).await?;
            use $crate::krate::sync::mapper::SyncOperator as _;
            $app.put(result.records, $hist).await?;
            use $crate::krate::sync::mapper::SyncMapper as _;
            if result_len < page_size {
                $app.insert_sync_log($crate::krate::sync::po::SyncLogTransient {
                    remote_id: shake_rsp.instance_id.clone().try_into()?,
                    table_name: $crate::krate::sync::dto::SyncTableEnum::$st
                        .to_string()
                        .try_into()?,
                    end_sync_in: sync_time,
                    start_tid_ex: initial_start,
                    sync_finish_tid: TID::default(),
                })
                .await?;
                break;
            }
        }
    };
    ($app:ident, $st:ident, $endpoint:ident, $hist:expr) => {
        sync_one!($app, $st, $endpoint, $hist, async |_, _, _| {
            Ok::<(), anyhow::Error>(())
        })
    };
}
