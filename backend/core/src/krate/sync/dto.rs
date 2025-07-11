use chin_sql::time_type::TID;
use serde::Serialize;

use crate::mapper::TheSameKey;

pub struct SyncShakeReq {
    pub client_id: String,
    pub client_app_version: String,
    pub server_id: String,
}

pub struct SyncShakeRsp {
    pub stop: bool,
    pub last_sync_time: Option<TID>,
}

pub struct SyncSendSameKeyReq {
    pub sync_id: String,
    pub table_name: String,
    pub same_keys: Vec<TheSameKey>,
}

pub struct SyncSendSameKeyRsp {}

pub struct SyncFetchAbsentReq {
    pub sync_id: String,
    pub table_name: String,
    pub page_size: usize,
}

pub struct SyncFetchAbsentRsp<T: Serialize> {
    pub records: Vec<T>,
}
