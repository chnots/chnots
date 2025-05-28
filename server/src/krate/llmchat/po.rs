use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use kdb_derives::KdbSqlInserter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct LLMChatBot {
    #[gts_primary]
    #[gts_length = 500]
    pub(crate) id: String,
    #[gts_length = 500]
    pub(crate) name: String,
    pub(crate) body: String,
    pub(crate) svg_logo: Option<String>,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct LLMChatTemplate {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 200]
    pub(crate) name: String,
    pub(crate) prompt: String,
    pub(crate) svg_logo: Option<String>,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct LLMChatSession {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) template_id: String,
    #[gts_length = 200]
    pub(crate) title: String,
    #[gts_length = 200]
    pub(crate) kspace: String,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct LLMChatRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) session_id: String,
    #[gts_length = 40]
    pub(crate) pre_record_id: Option<String>,
    pub(crate) content: String,
    pub(crate) reasoning_content: String,
    #[gts_length = 40]
    pub(crate) role: String,
    #[gts_length = 40]
    pub(crate) role_id: Option<String>, // maybe bot id
    pub(crate) insert_time: DateTime<FixedOffset>,
    pub(crate) omit_time: Option<DateTime<FixedOffset>>,
}
