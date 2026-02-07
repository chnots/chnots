use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use chin_sql::{
    str_type::{Text, Varchar},
    time_type::TID,
};
use chin_tools::SharedStr;
use serde::{Deserialize, Serialize};

use crate::{
    krate::sync::po::{SyncAllEndpoints, SyncEndpoint},
    model::{
        OtidTableSupport, SidTableEnum, SidTableSupport,
        otid_table::{OtidTableEnum, OtidWithEnum, OtidWithGeneric},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncInfo<T> {
    pub instance_id: SharedStr,
    pub start_ex: TID,
    pub end_in: TID,
    pub table_type: PhantomData<T>,
    pub pantient: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SyncSingleStep {
    Omit,
    Data,
}

impl<T: OtidTableSupport> SyncInfo<T> {
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
    pub sync_step: SyncSingleStep,
}

impl<T: OtidTableSupport> Deref for SyncPageInfo<T> {
    type Target = SyncInfo<T>;

    fn deref(&self) -> &Self::Target {
        &self.sync_info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeDto {
    pub instance_id: SharedStr,
    pub db_version: String,
}

pub type SyncShakeReq = OtidWithEnum<SyncShakeDto>;
pub type SyncShakeArg<T> = OtidWithGeneric<SyncShakeDto, T>;

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
pub(crate) enum SyncTIDListPage {
    StartEnd {
        start_ex: TID,
        end_in: TID,
        page_size: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SyncTIDListDto {
    pub page: SyncTIDListPage,
    pub hist: bool,
}

pub type SyncOtidTIDListReq = OtidWithEnum<SyncTIDListDto>;
pub type SyncTIDListArg<T> = OtidWithGeneric<SyncTIDListDto, T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOtidTIDListRsp {
    pub data: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDataDto<T> {
    pub(crate) cmds: Vec<SyncDataOperation<T>>,
    pub(crate) max_tid: TID,
    pub(crate) nomore: bool,
}

pub type SyncDataReqRsp = OtidWithEnum<SyncDataDto<String>>;

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
        let res: Result<Vec<SyncDataOperation<$st>>, anyhow::Error> = $cmds
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
                                let c: $st = serde_json::from_str::<$st>(&data)?;
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
pub struct SyncEndpointCommitReq {
    pub data: SyncAllEndpoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEndpointCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEndpointListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEndpointListRsp {
    pub data: SyncAllEndpoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEndpointSyncReq {
    pub endpoint: SyncEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEndpointSyncRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSidPoListReq<const LIMIT: usize> {
    pub pids: Vec<Varchar<LIMIT>>,
    pub table_type: SidTableEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSidPoCommitReq {
    pub pos: Vec<String>,
    pub table_type: SidTableEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSidPoCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSidPoGenericDto<T> {
    pub pos: Vec<T>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSidPoEnumDto {
    pub pos: Vec<String>,
    pub table_type: SidTableEnum,
}

impl<T: SidTableSupport> TryFrom<SyncSidPoGenericDto<T>> for SyncSidPoEnumDto {
    type Error = anyhow::Error;

    fn try_from(value: SyncSidPoGenericDto<T>) -> Result<Self, Self::Error> {
        let col: Result<Vec<String>, serde_json::Error> =
            value.pos.iter().map(|e| serde_json::to_string(e)).collect();
        Ok(Self {
            pos: col?,
            table_type: T::get_sid_enum(),
        })
    }
}
