use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBot {
    pub id: String,
    pub name: String,
    pub body: String,
    pub svg_logo: Option<String>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplate {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub svg_logo: Option<String>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSession {
    pub id: String,
    pub bot_id: String,
    pub template_id: String,
    pub title: String,
    pub namespace: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatRecord {
    pub id: String,
    pub session_id: String,
    pub pre_record_id: Option<String>,
    pub content: String,
    pub role: String,
    pub insert_time: DateTime<FixedOffset>, 
}