use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfig {
    pub id: String,
    pub config: String,

    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
    pub update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplate {
    pub id: String,
    pub template: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
    pub update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryV1 {
    pub id: String,
    pub title: String,
    pub serie_id: String,
    pub template_id: String,
    pub llm_id: String,
    pub role: String,
    pub content: String,
    pub insert_time: DateTime<FixedOffset>,
}
