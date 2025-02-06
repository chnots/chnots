use serde::{Deserialize, Serialize};

use crate::model::db::llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate};

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
    pub(crate) templates: Vec<LLMChatTemplate>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListSessionReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatListSessionRsp {
    pub(crate) sessions: Vec<LLMChatSession>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatUpdateSessionReq {
    pub title: Option<String>,
    pub session_id: String,

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatUpdateSessionRsp {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTruncateSessionReq {
    pub record_id_included: String,
    pub session_id: String,

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTruncateSessionRsp {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionDetialReq {
    pub(crate) session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionDetailRsp {
    pub(crate) records: Vec<LLMChatRecord>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteBotReq {
    pub(crate) bot_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteBotRsp {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteTemplateReq {
    pub(crate) template_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteTemplateRsp {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteSessionReq {
    pub(crate) session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatDeleteSessionRsp {}
