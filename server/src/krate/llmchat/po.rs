use chin_sql::str_type::{Text, Varchar};
use chin_sql::time_type::TID;
use chin_sql::{ GenerateTableSchema};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct LLMChatBot {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    pub(crate) name: Varchar<500>,
    pub(crate) body: Text,
    pub(crate) svg_logo: Option<Text>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct LLMChatTemplate {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    pub(crate) name: Varchar<200>,
    pub(crate) prompt: Text,
    pub(crate) svg_logo: Option<Text>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct LLMChatSession {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub(crate) template_tid: TID,
    pub(crate) title: Varchar<200>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
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
    pub(crate) content: Text,
    pub(crate) reasoning_content: Text,
    pub(crate) role: Varchar<40>,
    #[gts_type = "i64"]
    pub(crate) role_id: Option<TID>, // maybe bot tid
}
