use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct LLMChatBot {
    #[gts_primary]
    #[gts_length = 500]
    pub id: String,
    #[gts_length = 500]
    pub name: String,
    pub body: String,
    pub svg_logo: Option<String>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct LLMChatTemplate {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 200]
    pub name: String,
    pub prompt: String,
    pub svg_logo: Option<String>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct LLMChatSession {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub bot_id: String,
    #[gts_length = 40]
    pub template_id: String,
    #[gts_length = 200]
    pub title: String,
    #[gts_length = 200]
    pub namespace: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct LLMChatRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub session_id: String,
    #[gts_length = 40]
    pub pre_record_id: Option<String>,
    pub content: String,
    #[gts_length = 40]
    pub role: String,
    #[gts_length = 40]
    pub role_id: Option<String>, // maybe bot id
    pub insert_time: DateTime<FixedOffset>,
    pub omit_time: Option<DateTime<FixedOffset>>,
}
