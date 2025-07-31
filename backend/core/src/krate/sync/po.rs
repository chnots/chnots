use chin_sql::{GenerateTableSchema, str_type::Varchar, time_type::TID};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, GenerateTableSchema)]
pub struct SyncLogTransient {
    pub(crate) remote_id: Varchar<100>,
    pub(crate) table_name: Varchar<100>,
    #[gts_type = "i64"]
    pub(crate) end_sync_in: TID,
    #[gts_type = "i64"]
    pub(crate) start_tid_ex: TID,
    #[gts_type = "i64"]
    pub(crate) sync_finish_tid: TID,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncEndpoint {
    pub(crate) ip: String,
    pub(crate) port: u16,
}

impl SyncEndpoint {
    pub fn to_url(&self, url: &str) -> String {
        let link = format!("http://{}:{}/{}", self.ip, self.port, url.strip_prefix("/").unwrap_or("unknown-point"));
        link
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncAllEndpoints {
    pub(crate) endpoints: Vec<SyncEndpoint>,
}

#[derive(Debug, Clone, Copy)]
pub enum RecordState {
    Absent = 0,
    Cur = 1,
    Hist = 2,
}

impl RecordState {
    pub fn as_num(&self) -> i8 {
        *self as i8
    }
}

impl TryFrom<i32> for RecordState {
    type Error = anyhow::Error;

    fn try_from(x: i32) -> Result<Self, Self::Error> {
        match x {
            x if x == RecordState::Absent as i32 => Ok(RecordState::Absent),
            x if x == RecordState::Cur as i32 => Ok(RecordState::Cur),
            x if x == RecordState::Hist as i32 => Ok(RecordState::Hist),
            _ => anyhow::bail!("unable convert from i8 for RecordState"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TidCompare {
    pub(crate) tid: TID,
    pub(crate) lstate: RecordState,
    pub(crate) rstate: RecordState,
}
