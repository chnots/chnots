use chin_sql::str_type::{Text, Varchar};
use chin_sql::time_type::TID;
use chin_sql::{ GenerateTableSchema};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct LLMChatBot {
    #[gts_primary]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,

    pub name: Varchar<500>,
    pub body: Text,
    pub svg_logo: Option<Text>,
    pub update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct LLMChatTemplate {
    #[gts_primary]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    pub name: Varchar<200>,
    pub prompt: Text,
    pub svg_logo: Option<Text>,
    pub update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct LLMChatSession {
    #[gts_primary]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub template_tid: TID,
    pub title: Varchar<200>,
    pub update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct LLMChatRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub session_tid: TID,
    #[gts_type = "i64"]
    pub pre_record_tid: Option<TID>,
    pub content: Text,
    pub reasoning_content: Text,
    pub role: Varchar<40>,
    #[gts_type = "i64"]
    pub role_id: Option<TID>, // maybe bot tid
}
