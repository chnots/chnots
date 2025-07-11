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

pub struct SyncFetchSameKeyReq {
    pub sync_id: String,
    pub table_name: String,
    pub start_tid_ex: String,
}

pub struct SyncFetchSameKeyRsp {
    pub same_keys: Vec<TheSameKey>,
}

pub struct SyncFetchAbsentReq {
    pub table_name: String,
    pub tids: Vec<TID>,
}

pub struct SyncFetchAbsentRsp<T: Serialize> {
    pub records: Vec<T>,
}

pub(crate) enum FetchDataType {
    RangePage {
        start_ex: TID,
        end_in: TID,
        page_size: usize,
    },
    Tids (Vec<TID>)
}
