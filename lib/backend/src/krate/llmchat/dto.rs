use chin_sql::time_type::TID;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatOverwriteBotReq {
    pub bot: LLMChatBot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatOverwriteBotRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatOverwriteTemplateReq {
    pub template: LLMChatTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatOverwriteTemplateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatInsertSessionReq {
    pub session: LLMChatSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatInsertSessionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatInsertRecordReq {
    pub record: LLMChatRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatInsertRecordRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListBotReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListBotRsp {
    pub bots: Vec<LLMChatBot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListTemplateReq {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListTemplateRsp {
    pub templates: Vec<LLMChatTemplate>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListSessionReq {
    pub session_otid: Option<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListSessionRsp {
    pub sessions: Vec<LLMChatSession>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatUpdateSessionReq {
    pub title: Option<String>,
    pub delete: Option<bool>,
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatUpdateSessionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTruncateSessionReq {
    pub remove_rid_included: TID,
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTruncateSessionRsp {
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionDetialReq {
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionDetailRsp {
    pub session: Option<LLMChatSession>,
    pub records: Vec<LLMChatRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteBotReq {
    pub bot_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteBotRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteTemplateReq {
    pub template_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteTemplateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteSessionReq {
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteSessionRsp {}
