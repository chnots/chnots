use chin_sql::{GenerateTableSchema, str_type::Varchar, time_type::TID};
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
