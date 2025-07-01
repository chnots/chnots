use chin_sql::{str_type::Varchar, time_type::TID};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatOverwriteBotReq {
    pub(crate) bot: LLMChatBot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatOverwriteBotRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatOverwriteTemplateReq {
    pub(crate) template: LLMChatTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatOverwriteTemplateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatInsertSessionReq {
    pub(crate) session: LLMChatSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatInsertSessionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatInsertRecordReq {
    pub(crate) record: LLMChatRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatInsertRecordRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListBotReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListBotRsp {
    pub(crate) bots: Vec<LLMChatBot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListTemplateReq {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListTemplateRsp {
    pub(crate) templates: Vec<LLMChatTemplate>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListSessionReq {
    pub(crate) session_tid: Option<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatListSessionRsp {
    pub(crate) sessions: Vec<LLMChatSession>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatUpdateSessionReq {
    pub(crate) title: Option<Varchar<200>>,
    pub(crate) delete: Option<bool>,
    pub(crate) session_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatUpdateSessionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatTruncateSessionReq {
    pub(crate) remove_rid_included: TID,
    pub(crate) session_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatTruncateSessionRsp {
    pub(crate) count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatSessionDetialReq {
    pub(crate) session_tid: TID,
    pub(crate) with_omit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatSessionDetailRsp {
    pub(crate) session: Option<LLMChatSession>,
    pub(crate) records: Vec<LLMChatRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteBotReq {
    pub(crate) bot_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteBotRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteTemplateReq {
    pub(crate) template_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteTemplateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteSessionReq {
    pub(crate) session_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LLMChatDeleteSessionRsp {}
