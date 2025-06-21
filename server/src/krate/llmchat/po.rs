use chin_sql::time_type::TID;
use chin_sql::{ChinSqlCrud, GenerateTableSchema};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct LLMChatBot {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    #[gts_length = 500]
    pub(crate) name: String,
    pub(crate) body: String,
    pub(crate) svg_logo: Option<String>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct LLMChatTemplate {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    #[gts_length = 200]
    pub(crate) name: String,
    pub(crate) prompt: String,
    pub(crate) svg_logo: Option<String>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct LLMChatSession {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub(crate) template_tid: TID,
    #[gts_length = 200]
    pub(crate) title: String,
    #[gts_length = 200]
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct LLMChatRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub(crate) session_tid: TID,
    #[gts_type = "i64"]
    pub(crate) pre_record_tid: Option<TID>,
    pub(crate) content: String,
    pub(crate) reasoning_content: String,
    #[gts_length = 40]
    pub(crate) role: String,
    #[gts_type = "i64"]
    pub(crate) role_id: Option<TID>, // maybe bot tid
}
