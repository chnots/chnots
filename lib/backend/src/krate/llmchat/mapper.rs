use anyhow::Ok;
use chin_tools::{AResult, EResult, utils::sort_util};

use crate::{expand_mt_branch, mapper::MapperType, model::dto::KReq};

use super::*;

pub trait LLMChatMapper {
    async fn llmchat_bot_commit(
        &self,
        req: KReq<LLMChatBotCommitReq>,
    ) -> AResult<LLMChatBotCommitRsp>;

    async fn llmchat_template_commit(
        &self,
        req: KReq<LLMChatTemplateCommitReq>,
    ) -> AResult<LLMChatTemplateCommitRsp>;

    async fn llmchat_session_commit(
        &self,
        req: KReq<LLMChatSessionCommitReq>,
    ) -> AResult<LLMChatSessionCommitRsp>;

    async fn llmchat_record_commit(
        &self,
        req: KReq<LLMChatRecordCommitReq>,
    ) -> AResult<LLMChatRecordCommitRsp>;

    async fn llmchat_bot_list(&self, req: KReq<LLMChatBotListReq>) -> AResult<LLMChatBotListRsp>;

    async fn llmchat_template_list(
        &self,
        req: KReq<LLMChatTemplateListReq>,
    ) -> AResult<LLMChatTemplateListRsp>;

    async fn llmchat_session_list(
        &self,
        req: KReq<LLMChatSessionListReq>,
    ) -> AResult<LLMChatSessionListRsp>;

    async fn llmchat_session_record_fetch(
        &self,
        req: KReq<LLMChatSessionRecordFetchReq>,
    ) -> AResult<LLMChatSessionRecordFetchRsp>;

    async fn llmchat_session_record_truncate(
        &self,
        req: KReq<LLMChatSessionRecordTruncateReq>,
    ) -> AResult<LLMChatSessionRecordTruncateRsp>;

    async fn llmchat_bot_archive(
        &self,
        req: KReq<LLMChatBotArchiveReq>,
    ) -> AResult<LLMChatBotArchiveRsp>;

    async fn llmchat_template_archive(
        &self,
        req: KReq<LLMChatTemplateArchiveReq>,
    ) -> AResult<LLMChatTemplateArchiveRsp>;

    async fn llmchat_session_archive(
        &self,
        req: KReq<LLMChatSessionArchiveReq>,
    ) -> AResult<LLMChatSessionArchiveRsp>;

    async fn ensure_table_llm_chat(&self) -> EResult;
}

impl LLMChatMapper for MapperType {
    async fn llmchat_bot_commit(
        &self,
        req: KReq<super::LLMChatBotCommitReq>,
    ) -> AResult<super::LLMChatBotCommitRsp> {
        expand_mt_branch!(self.llmchat_bot_commit(req))
    }

    async fn llmchat_template_commit(
        &self,
        req: KReq<super::LLMChatTemplateCommitReq>,
    ) -> AResult<super::LLMChatTemplateCommitRsp> {
        expand_mt_branch!(self.llmchat_template_commit(req))
    }

    async fn llmchat_session_commit(
        &self,
        req: KReq<super::LLMChatSessionCommitReq>,
    ) -> AResult<super::LLMChatSessionCommitRsp> {
        expand_mt_branch!(self.llmchat_session_commit(req))
    }

    async fn llmchat_record_commit(
        &self,
        req: KReq<super::LLMChatRecordCommitReq>,
    ) -> AResult<super::LLMChatRecordCommitRsp> {
        expand_mt_branch!(self.llmchat_record_commit(req))
    }

    async fn llmchat_bot_list(
        &self,
        req: KReq<super::LLMChatBotListReq>,
    ) -> AResult<super::LLMChatBotListRsp> {
        expand_mt_branch!(self.llmchat_bot_list(req))
    }

    async fn llmchat_template_list(
        &self,
        req: KReq<super::LLMChatTemplateListReq>,
    ) -> AResult<super::LLMChatTemplateListRsp> {
        expand_mt_branch!(self.llmchat_template_list(req))
    }

    async fn llmchat_session_list(
        &self,
        req: KReq<super::LLMChatSessionListReq>,
    ) -> AResult<super::LLMChatSessionListRsp> {
        expand_mt_branch!(self.llmchat_session_list(req))
    }

    async fn llmchat_session_record_fetch(
        &self,
        req: KReq<super::LLMChatSessionRecordFetchReq>,
    ) -> AResult<super::LLMChatSessionRecordFetchRsp> {
        let mut raw_result = expand_mt_branch!(self.llmchat_session_record_fetch(req))?;

        sort_util::sort_by_prev(
            &mut raw_result.records,
            false,
            |r| &r.otid,
            |r| &r.pre_record_otid,
            |e| &e.otid,
        );

        Ok(raw_result)
    }

    async fn llmchat_bot_archive(
        &self,
        req: KReq<super::LLMChatBotArchiveReq>,
    ) -> AResult<super::LLMChatBotArchiveRsp> {
        expand_mt_branch!(self.llmchat_bot_archive(req))
    }

    async fn llmchat_template_archive(
        &self,
        req: KReq<super::LLMChatTemplateArchiveReq>,
    ) -> AResult<super::LLMChatTemplateArchiveRsp> {
        expand_mt_branch!(self.llmchat_template_archive(req))
    }

    async fn llmchat_session_archive(
        &self,
        req: KReq<super::LLMChatSessionArchiveReq>,
    ) -> AResult<super::LLMChatSessionArchiveRsp> {
        expand_mt_branch!(self.llmchat_session_archive(req))
    }

    async fn ensure_table_llm_chat(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat())
    }

    async fn llmchat_session_record_truncate(
        &self,
        req: KReq<LLMChatSessionRecordTruncateReq>,
    ) -> AResult<LLMChatSessionRecordTruncateRsp> {
        expand_mt_branch!(self.llmchat_session_record_truncate(req))
    }
}
