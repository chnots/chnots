use chin_sql::SqlInserter;
use chin_tools::{
    utils::sort_util::sort_by_prev,
    wrapper::anyhow::{AResult, EResult},
};
use chrono::Local;

use super::{
    sql::{SqlSegBuilder, SqlUpdater, Wheres},
    KDb, KDbBehaiver, KDbConnBehaiver, KDbRow,
};
use crate::{
    mapper::LLMChatMapper,
    model::{
        db::llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        dto::{llmchat::*, KReq},
    },
};

use super::DeserializeMapper;

impl LLMChatMapper for KDb {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<LLMChatOverwriteBotReq>,
    ) -> AResult<LLMChatOverwriteBotRsp> {
        let bot = &req.bot;
        let inserter = SqlInserter::new(LLMChatBot::table_name())
            .fields(LLMChatBot::field_id(), &bot.id)
            .fields(LLMChatBot::field_name(), &bot.name)
            .fields(LLMChatBot::field_body(), &bot.body)
            .fields(LLMChatBot::field_svg_logo(), bot.svg_logo.as_ref())
            .fields(LLMChatBot::field_insert_time(), &bot.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        let tmpl = &req.template;
        let inserter = SqlInserter::new(LLMChatTemplate::table_name())
            .fields(LLMChatTemplate::field_id(), &tmpl.id)
            .fields(LLMChatTemplate::field_name(), &tmpl.name)
            .fields(LLMChatTemplate::field_prompt(), &tmpl.prompt)
            .fields(LLMChatTemplate::field_svg_logo(), tmpl.svg_logo.as_ref())
            .fields(LLMChatTemplate::field_insert_time(), &tmpl.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let title: String = req.session.title.chars().into_iter().take(300).collect();
        let session = &req.session;
        let inserter = SqlInserter::new(LLMChatSession::table_name())
            .fields(LLMChatSession::field_id(), &session.id)
            .fields(LLMChatSession::field_bot_id(), &session.bot_id)
            .fields(LLMChatSession::field_template_id(), &session.template_id)
            .fields(LLMChatSession::field_title(), &title)
            .fields(LLMChatSession::field_namespace(), &session.namespace)
            .fields(LLMChatSession::field_insert_time(), &session.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        let rec = &req.record;
        let inserter = SqlInserter::new(LLMChatRecord::table_name())
            .fields(LLMChatRecord::field_id(), &rec.id)
            .fields(LLMChatRecord::field_session_id(), &rec.session_id)
            .fields(
                LLMChatRecord::field_pre_record_id(),
                rec.pre_record_id.as_ref(),
            )
            .fields(LLMChatRecord::field_content(), &rec.content)
            .fields(LLMChatRecord::field_role(), &rec.role)
            .fields(LLMChatRecord::field_role_id(), rec.role_id.as_ref())
            .fields(LLMChatRecord::field_insert_time(), &rec.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_bot")
            .r#where(Wheres::and([Wheres::is_null("delete_time")]))
            .raw("order by insert_time desc");

        let bots = self
            .conn()
            .await?
            .qry_list(query, |e| KDbRow::to_llmchat_bot(e))
            .await?
            .into_iter()
            .collect();

        Ok(LLMChatListBotRsp { bots })
    }

    async fn llm_chat_list_templates(
        &self,
        req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_template")
            .r#where(Wheres::and([Wheres::is_null("delete_time")]))
            .raw("order by insert_time desc");

        let templates: Vec<LLMChatTemplate> = self
            .conn()
            .await?
            .qry_list(query, |e| e.to_llmchat_template())
            .await?;

        Ok(LLMChatListTemplateRsp { templates })
    }

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<LLMChatListSessionReq>,
    ) -> AResult<LLMChatListSessionRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_session")
            .r#where(Wheres::and([
                Wheres::is_null("delete_time"),
                Wheres::equal("namespace", req.namespace),
            ]))
            .raw("order by insert_time desc");

        let sessions = self
            .conn()
            .await?
            .qry_list(query, |e| e.to_llmchat_session())
            .await?;

        Ok(LLMChatListSessionRsp { sessions })
    }

    async fn llm_chat_session_detail(
        &self,
        req: KReq<LLMChatSessionDetialReq>,
    ) -> AResult<LLMChatSessionDetailRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_record")
            .r#where(Wheres::and([
                Wheres::equal("session_id", req.session_id.clone()),
                Wheres::is_null("omit_time"),
            ]))
            .raw("order by insert_time desc");

        let records: Vec<LLMChatRecord> = self
            .conn()
            .await?
            .qry_list(query, |e| e.to_llmchat_record())
            .await?;

        Ok(LLMChatSessionDetailRsp { records })
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<LLMChatDeleteBotReq>,
    ) -> AResult<LLMChatDeleteBotRsp> {
        let updater = SqlUpdater::new("llm_chat_bot")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.bot_id));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        let updater = SqlUpdater::new("llm_chat_template")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.template_id));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = SqlUpdater::new("llm_chat_session")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.session_id));
        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        self.create_table(LLMChatBot::table_creation_sql(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        self.create_table(LLMChatTemplate::table_creation_sql(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        self.create_table(LLMChatSession::table_creation_sql(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        self.create_table(LLMChatRecord::table_creation_sql(self.db_type()))
            .await?;
        Ok(())
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp> {
        let updater = SqlUpdater::new("llm_chat_session")
            .set_if_some("title", req.title.as_ref())
            .trans_if_some("delete_time", req.delete, |flag| {
                if flag {
                    Some(Local::now().fixed_offset())
                } else {
                    None
                }
            })
            .r#where(Wheres::and([Wheres::equal("id", req.session_id.as_str())]));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatUpdateSessionRsp {})
    }

    async fn llm_chat_truncate_session(
        &self,
        req: KReq<LLMChatTruncateSessionReq>,
    ) -> AResult<LLMChatTruncateSessionRsp> {
        let mut records = self
            .llm_chat_session_detail(KReq {
                body: LLMChatSessionDetialReq {
                    session_id: req.session_id.clone(),
                },
                namespace: req.namespace.clone(),
            })
            .await?
            .records;

        sort_by_prev(
            &mut records,
            false,
            |r| &r.id,
            |r| &r.pre_record_id,
            |r| &r.insert_time,
        );

        let mut to_omit_ids = vec![];
        let mut remove_flag = false;

        for r in records {
            if r.id == req.remove_rid_included {
                remove_flag = true;
            }
            if remove_flag {
                to_omit_ids.push(r.id);
            }
        }

        let updater = SqlUpdater::new("llm_chat_record")
            .set("omit_time", Local::now().fixed_offset())
            .r#where(Wheres::r#in("id", to_omit_ids));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatTruncateSessionRsp {})
    }
}
