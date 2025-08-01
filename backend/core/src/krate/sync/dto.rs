use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use chin_sql::time_type::TID;
use chin_tools::SharedStr;
use serde::{Deserialize, Serialize};

use crate::{
    krate::sync::po::{SyncAllEndpoints, SyncEndpoint},
    model::{KOtidSupport, otid_table::OtidTableEnum},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncInfo<T> {
    pub instance_id: SharedStr,
    pub start_ex: TID,
    pub end_in: TID,
    pub table_type: PhantomData<T>,
}

impl<T: KOtidSupport> SyncInfo<T> {
    pub fn to_table_name(&self) -> String {
        format!(
            "sync_{}_{}",
            T::table_name(false),
            self.instance_id.as_str()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPageInfo<T> {
    pub sync_info: SyncInfo<T>,
    pub page_size: usize,
    pub start_ex: TID,
}

impl<T: KOtidSupport> Deref for SyncPageInfo<T> {
    type Target = SyncInfo<T>;

    fn deref(&self) -> &Self::Target {
        &self.sync_info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithEnum<E> {
    pub(crate) table_type: OtidTableEnum,
    pub(crate) dto: E,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithGer<E, T> {
    pub(crate) dto: E,
    pub(crate) table_type: PhantomData<T>,
}

impl<E, T> Deref for OtidWithGer<E, T> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.dto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeDto {
    pub instance_id: SharedStr,
    pub db_version: String,
}

pub type SyncShakeReq = OtidWithEnum<SyncShakeDto>;
pub type SyncShakeArg<T> = OtidWithGer<SyncShakeDto, T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncShakeRspEnum {
    NotSameVersion(String),
    SameClient,
    BeginSync { sync_time: TID },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeRsp {
    pub instance_id: SharedStr,
    pub data: SyncShakeRspEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum SyncFetchTIDPage {
    StartEnd {
        start_ex: TID,
        end_in: TID,
        page_size: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SyncFetchTIDDto {
    pub page: SyncFetchTIDPage,
    pub hist: bool,
}

pub type SyncFetchTIDReq = OtidWithEnum<SyncFetchTIDDto>;
pub type SyncFetchTIDArg<T> = OtidWithGer<SyncFetchTIDDto, T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFetchTIDRsp {
    pub data: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDataDto<T> {
    pub(crate) cmds: Vec<SyncDataOperation<T>>,
    pub(crate) max_tid: TID,
    pub(crate) nomore: bool,
}

pub type SyncDataReqRsp = OtidWithEnum<SyncDataDto<String>>;
pub type SyncDataArg<T> = SyncDataDto<T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDataOperation<T> {
    Omit(TID),
    Pull { tid: TID, hist: bool },
    Push { data: T, hist: bool },
}

#[macro_export]
macro_rules! sync_cmds_json_to_st {
    ($st:ty, $cmds:expr) => {{
        use $crate::krate::sync::dto::SyncDataOperation;
        let res: Result<Vec<SyncDataOperation<$st>>, serde_json::Error> = $cmds
            .into_iter()
            .map(|s| {
                let c = match s {
                    $crate::krate::sync::dto::SyncDataOperation::Omit(tid) => {
                        SyncDataOperation::Omit(tid)
                    }
                    $crate::krate::sync::dto::SyncDataOperation::Pull { tid, hist } => {
                        SyncDataOperation::Pull { tid, hist }
                    }
                    $crate::krate::sync::dto::SyncDataOperation::Push { data, hist } => {
                        SyncDataOperation::Push {
                            data: {
                                let c: $st = serde_json::from_str(&data)?;
                                c
                            },
                            hist,
                        }
                    }
                };
                Ok(c)
            })
            .collect();
        res
    }};
}

#[macro_export]
macro_rules! sync_cmds_st_to_json {
    ($cmds:expr) => {{
        use $crate::krate::sync::dto::SyncDataOperation;
        let cmds: AResult<Vec<SyncDataOperation<String>>> = $cmds
            .iter()
            .map(|c| {
                let d = match c {
                    SyncDataOperation::Omit(tid) => SyncDataOperation::Omit(tid.clone()),
                    SyncDataOperation::Pull { tid, hist } => SyncDataOperation::Pull {
                        tid: tid.to_owned(),
                        hist: hist.to_owned(),
                    },
                    SyncDataOperation::Push { data, hist } => SyncDataOperation::Push {
                        data: serde_json::to_string(&data)?,
                        hist: hist.to_owned(),
                    },
                };
                Ok(d)
            })
            .collect();
        cmds?
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAllEndpointsReq {
    pub data: SyncAllEndpoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAllEndpointsRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSyncAllEndpointsReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSyncAllEndpointsRsp {
    pub data: SyncAllEndpoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncToEndpointReq {
    pub endpoint: SyncEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncToEndpointRsp {}
