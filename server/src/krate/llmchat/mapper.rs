use anyhow::Ok;
use chin_tools::{utils::sort_util, AResult, EResult};

use crate::{
    expand_mt_branch,
    mapper::{dump::RecordCallbackType, MapperType},
    model::dto::KReq,
};

use super::*;

pub(crate) trait LLMChatDeserializeMapper {
    fn to_llmchat_bot(self) -> AResult<LLMChatBot>;
    fn to_llmchat_template(self) -> AResult<LLMChatTemplate>;
    fn to_llmchat_session(self) -> AResult<LLMChatSession>;
    fn to_llmchat_record(self) -> AResult<LLMChatRecord>;
}

pub(crate) trait LLMChatDumpMapper {
    async fn dump_llmchat_bot(&self, callback: &RecordCallbackType) -> EResult;
    async fn dump_llmchat_template(&self, callback: &RecordCallbackType) -> EResult;
    async fn dump_llmchat_session(&self, callback: &RecordCallbackType) -> EResult;
    async fn dump_llmchat_record(&self, callback: &RecordCallbackType) -> EResult;

    async fn dump_llmchat(&self, callback: &RecordCallbackType) -> EResult {
        self.dump_llmchat_bot(callback).await?;
        self.dump_llmchat_template(callback).await?;
        self.dump_llmchat_session(callback).await?;
        self.dump_llmchat_record(callback).await?;

        Ok(())
    }
}

pub(crate) trait LLMChatMapper {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<LLMChatOverwriteBotReq>,
    ) -> AResult<LLMChatOverwriteBotRsp>;

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp>;

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp>;

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp>;

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp>;

    async fn llm_chat_list_templates(
        &self,
        req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp>;

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<LLMChatListSessionReq>,
    ) -> AResult<LLMChatListSessionRsp>;

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp>;

    async fn llm_chat_session_detail(
        &self,
        req: KReq<LLMChatSessionDetialReq>,
    ) -> AResult<LLMChatSessionDetailRsp>;

    async fn llm_chat_truncate_session(
        &self,
        req: KReq<LLMChatTruncateSessionReq>,
    ) -> AResult<LLMChatTruncateSessionRsp>;

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<LLMChatDeleteBotReq>,
    ) -> AResult<LLMChatDeleteBotRsp>;

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp>;

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp>;

    async fn ensure_table_llm_chat_bot(&self) -> EResult;
    async fn ensure_table_llm_chat_template(&self) -> EResult;
    async fn ensure_table_llm_chat_session(&self) -> EResult;
    async fn ensure_table_llm_chat_record(&self) -> EResult;

    async fn ensure_table_llm_chat(&self) -> EResult {
        self.ensure_table_llm_chat_bot().await?;
        self.ensure_table_llm_chat_template().await?;
        self.ensure_table_llm_chat_session().await?;
        self.ensure_table_llm_chat_record().await?;

        Ok(())
    }
}

impl LLMChatMapper for MapperType {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<super::LLMChatOverwriteBotReq>,
    ) -> AResult<super::LLMChatOverwriteBotRsp> {
        expand_mt_branch!(self.llm_chat_overwrite_bot(req))
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<super::LLMChatOverwriteTemplateReq>,
    ) -> AResult<super::LLMChatOverwriteTemplateRsp> {
        expand_mt_branch!(self.llm_chat_overwrite_template(req))
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<super::LLMChatInsertSessionReq>,
    ) -> AResult<super::LLMChatInsertSessionRsp> {
        expand_mt_branch!(self.llm_chat_insert_session(req))
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<super::LLMChatInsertRecordReq>,
    ) -> AResult<super::LLMChatInsertRecordRsp> {
        expand_mt_branch!(self.llm_chat_insert_record(req))
    }

    async fn llm_chat_list_bots(
        &self,
        req: KReq<super::LLMChatListBotReq>,
    ) -> AResult<super::LLMChatListBotRsp> {
        expand_mt_branch!(self.llm_chat_list_bots(req))
    }

    async fn llm_chat_list_templates(
        &self,
        req: KReq<super::LLMChatListTemplateReq>,
    ) -> AResult<super::LLMChatListTemplateRsp> {
        expand_mt_branch!(self.llm_chat_list_templates(req))
    }

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<super::LLMChatListSessionReq>,
    ) -> AResult<super::LLMChatListSessionRsp> {
        expand_mt_branch!(self.llm_chat_list_sessions(req))
    }

    async fn llm_chat_session_detail(
        &self,
        req: KReq<super::LLMChatSessionDetialReq>,
    ) -> AResult<super::LLMChatSessionDetailRsp> {
        let mut raw_result = expand_mt_branch!(self.llm_chat_session_detail(req))?;

        sort_util::sort_by_prev(
            &mut raw_result.records,
            false,
            |r| &r.tid,
            |r| &r.pre_record_tid,
            |e| &e.tid,
        );

        Ok(raw_result)
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp> {
        expand_mt_branch!(self.llm_chat_update_session(req))
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<super::LLMChatDeleteBotReq>,
    ) -> AResult<super::LLMChatDeleteBotRsp> {
        expand_mt_branch!(self.llm_chat_delete_bot(req))
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<super::LLMChatDeleteTemplateReq>,
    ) -> AResult<super::LLMChatDeleteTemplateRsp> {
        expand_mt_branch!(self.llm_chat_delete_template(req))
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<super::LLMChatDeleteSessionReq>,
    ) -> AResult<super::LLMChatDeleteSessionRsp> {
        expand_mt_branch!(self.llm_chat_delete_session(req))
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_record())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_template())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_session())
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_bot())
    }

    async fn llm_chat_truncate_session(
        &self,
        req: KReq<LLMChatTruncateSessionReq>,
    ) -> AResult<LLMChatTruncateSessionRsp> {
        expand_mt_branch!(self.llm_chat_truncate_session(req))
    }
}
