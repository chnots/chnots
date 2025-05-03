use chin_sql::SqlInserter;
use chin_tools::{
    utils::sort_util::sort_by_prev,
    wrapper::anyhow::{AResult, EResult},
};
use chrono::Local;

use super::{
    sql::{SqlReader, SqlUpdater, Wheres},
    KDb, KDbBehaiver, KDbConnBehaiver, KDbRow,
};
use crate::{
    mapper::LLMChatMapper,
    model::{
        db::llmchat::{
            LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate,
        },
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
        let inserter = SqlInserter::new(LLMChatBot::TABLE)
            .fields(LLMChatBot::ID, &bot.id)
            .fields(LLMChatBot::NAME, &bot.name)
            .fields(LLMChatBot::BODY, &bot.body)
            .fields(LLMChatBot::SVG_LOGO, bot.svg_logo.as_ref())
            .fields(LLMChatBot::INSERT_TIME, &bot.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        let tmpl = &req.template;
        let inserter = SqlInserter::new(LLMChatTemplate::TABLE)
            .fields(LLMChatTemplate::ID, &tmpl.id)
            .fields(LLMChatTemplate::NAME, &tmpl.name)
            .fields(LLMChatTemplate::PROMPT, &tmpl.prompt)
            .fields(LLMChatTemplate::SVG_LOGO, tmpl.svg_logo.as_ref())
            .fields(LLMChatTemplate::INSERT_TIME, &tmpl.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let title: String = req.session.title.chars().into_iter().take(300).collect();
        let session = &req.session;
        let inserter = SqlInserter::new(LLMChatSession::TABLE)
            .fields(LLMChatSession::ID, &session.id)
            .fields(LLMChatSession::BOT_ID, &session.bot_id)
            .fields(LLMChatSession::TEMPLATE_ID, &session.template_id)
            .fields(LLMChatSession::TITLE, &title)
            .fields(LLMChatSession::NAMESPACE, &session.namespace)
            .fields(LLMChatSession::INSERT_TIME, &session.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        let rec = &req.record;
        let inserter = SqlInserter::new(LLMChatRecord::TABLE)
            .fields(LLMChatRecord::ID, &rec.id)
            .fields(LLMChatRecord::SESSION_ID, &rec.session_id)
            .fields(
                LLMChatRecord::PRE_RECORD_ID,
                rec.pre_record_id.as_ref(),
            )
            .fields(LLMChatRecord::CONTENT, &rec.content)
            .fields(LLMChatRecord::ROLE, &rec.role)
            .fields(LLMChatRecord::ROLE_ID, rec.role_id.as_ref())
            .fields(LLMChatRecord::INSERT_TIME, &rec.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let query = SqlReader::read_all(LLMChatBot::TABLE)
            .r#where(Wheres::and([Wheres::is_null(LLMChatBot::DELETE_TIME)]))
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
        let query = SqlReader::read_all(LLMChatTemplate::TABLE)
            .r#where(Wheres::and([Wheres::is_null(
                LLMChatTemplate::DELETE_TIME,
            )]))
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
        let query = SqlReader::read_all(LLMChatSession::TABLE)
            .r#where(Wheres::and([
                Wheres::is_null(LLMChatSession::DELETE_TIME),
                Wheres::equal(LLMChatSession::NAMESPACE, req.namespace),
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
        let query = SqlReader::read_all(LLMChatRecord::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(LLMChatRecord::SESSION_ID, req.session_id.clone()),
                Wheres::is_null(LLMChatRecord::OMIT_TIME),
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
        let updater = SqlUpdater::new(LLMChatBot::TABLE)
            .set(LLMChatBot::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.bot_id));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        let updater = SqlUpdater::new(LLMChatTemplate::TABLE)
            .set(
                LLMChatTemplate::DELETE_TIME,
                Local::now().fixed_offset(),
            )
            .r#where(Wheres::equal("id", &req.template_id));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = SqlUpdater::new(LLMChatSession::TABLE)
            .set(
                LLMChatSession::DELETE_TIME,
                Local::now().fixed_offset(),
            )
            .r#where(Wheres::equal(LLMChatSession::ID, &req.session_id));
        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        self.create_table(LLMChatBot::schema(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        self.create_table(LLMChatTemplate::schema(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        self.create_table(LLMChatSession::schema(self.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        self.create_table(LLMChatRecord::schema(self.db_type()))
            .await?;
        Ok(())
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp> {
        let updater = SqlUpdater::new(LLMChatSession::TABLE)
            .set_if_some(LLMChatSession::TITLE, req.title.as_ref())
            .trans_if_some(LLMChatSession::DELETE_TIME, req.delete, |flag| {
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

        let updater = SqlUpdater::new(LLMChatRecord::TABLE)
            .set(LLMChatRecord::OMIT_TIME, Local::now().fixed_offset())
            .r#where(Wheres::r#in("id", to_omit_ids));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatTruncateSessionRsp {})
    }
}
