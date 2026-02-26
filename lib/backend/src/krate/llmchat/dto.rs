use chin_sql::time_type::TID;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotCommitReq {
    pub bot: LLMChatBot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateCommitReq {
    pub template: LLMChatTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionCommitReq {
    pub session: LLMChatSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatRecordCommitReq {
    pub record: LLMChatRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatRecordCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotListRsp {
    pub bots: Vec<LLMChatBot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateListReq {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateListRsp {
    pub templates: Vec<LLMChatTemplate>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionListReq {
    pub session_otid: Option<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionListRsp {
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
pub struct LLMChatSessionRecordTruncateReq {
    pub remove_otid_included: TID,
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionRecordTruncateRsp {
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionRecordFetchReq {
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionRecordFetchRsp {
    pub session: Option<LLMChatSession>,
    pub records: Vec<LLMChatRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotArchiveReq {
    pub bot_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatBotArchiveRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateArchiveReq {
    pub template_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateArchiveRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionArchiveReq {
    pub session_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatSessionArchiveRsp {}
