use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::{FixedOffset, Local, Utc};

use crate::{
    mapper::LLMChatMapper,
    model::{
        db::llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        dto::llmchat::*,
    },
    to_sql,
    util::{
        sort_util::sort_by_prev,
        sql_builder::{PlaceHolderType, SqlSegBuilder, SqlUpdater, Wheres},
    },
};

use super::{KReq, Postgres};

impl LLMChatMapper for Postgres {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<LLMChatOverwriteBotReq>,
    ) -> AResult<LLMChatOverwriteBotRsp> {
        self.client().await?.execute(
            "insert into llm_chat_bot(id, name, body, insert_time) values($1, $2, $3, $4) on CONFLICT (id) DO UPDATE SET update_time = $2",
            &[
                &req.bot.id,
                &req.bot.name,
                &req.bot.body,
                &req.bot.insert_time
            ]
        ).await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        self.client().await?.execute(
            "insert into llm_chat_template(id, name, prompt, insert_time) values($1, $2, $3, $4) on CONFLICT (id) DO UPDATE SET update_time = $2",
            &[
                &req.template.id,
                &req.template.name,
                &req.template.prompt,
                &req.template.insert_time
            ]
        ).await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let title: String = req.session.title.chars().into_iter().take(300).collect();
        self.client().await?.execute(
            "insert into llm_chat_session(id, bot_id, template_id, title, namespace, insert_time) values($1, $2, $3, $4, $5, $6)",
            &[
                &req.session.id,
                &req.session.bot_id,
                &req.session.template_id,
                &title,
                &req.session.namespace,
                &req.session.insert_time
            ]
        ).await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        self.client().await?.execute(
            "insert into llm_chat_record(id, session_id, pre_record_id, content, role, role_id, insert_time) values($1, $2, $3, $4, $5, $6, $7)",
            &[
                &req.record.id,
                &req.record.session_id,
                &req.record.pre_record_id,
                &req.record.content,
                &req.record.role,
                &req.record.role_id,
                &req.record.insert_time
            ]
        ).await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_bot")
            .r#where(Wheres::and([Wheres::is_null("delete_time")]))
            .raw("order by insert_time desc")
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let bots: AResult<Vec<LLMChatBot>> = self
            .client()
            .await?
            .query(query.seg.as_str(), to_sql!(query.values))
            .await?
            .iter()
            .map(|t| {
                let r = LLMChatBot {
                    id: t.try_get("id")?,
                    name: t.try_get("name")?,
                    body: t.try_get("body")?,
                    delete_time: t.try_get("delete_time")?,
                    update_time: t.try_get("update_time")?,
                    insert_time: t.try_get("insert_time")?,
                    svg_logo: t.try_get("svg_logo")?,
                };
                Ok(r)
            })
            .collect();

        Ok(LLMChatListBotRsp { bots: bots? })
    }

    async fn llm_chat_list_templates(
        &self,
        req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from llm_chat_template")
            .r#where(Wheres::and([Wheres::is_null("delete_time")]))
            .raw("order by insert_time desc")
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let templates: AResult<Vec<LLMChatTemplate>> = self
            .client()
            .await?
            .query(query.seg.as_str(), to_sql!(query.values))
            .await?
            .iter()
            .map(|t| {
                let r = LLMChatTemplate {
                    id: t.try_get("id")?,
                    name: t.try_get("name")?,
                    delete_time: t.try_get("delete_time")?,
                    update_time: t.try_get("update_time")?,
                    insert_time: t.try_get("insert_time")?,
                    prompt: t.try_get("prompt")?,
                    svg_logo: t.try_get("svg_logo")?,
                };
                Ok(r)
            })
            .collect();

        Ok(LLMChatListTemplateRsp {
            templates: templates?,
        })
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
            .raw("order by insert_time desc")
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let sessions: AResult<Vec<LLMChatSession>> = self
            .client()
            .await?
            .query(query.seg.as_str(), to_sql!(query.values))
            .await?
            .iter()
            .map(|t| {
                let r = LLMChatSession {
                    id: t.try_get("id")?,
                    delete_time: t.try_get("delete_time")?,
                    update_time: t.try_get("update_time")?,
                    insert_time: t.try_get("insert_time")?,
                    bot_id: t.try_get("bot_id")?,
                    template_id: t.try_get("template_id")?,
                    title: t.try_get("title")?,
                    namespace: t.try_get("namespace")?,
                };
                Ok(r)
            })
            .collect();

        Ok(LLMChatListSessionRsp {
            sessions: sessions?,
        })
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
            .raw("order by insert_time desc")
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let records: AResult<Vec<LLMChatRecord>> = self
            .client()
            .await?
            .query(query.seg.as_str(), to_sql!(query.values))
            .await?
            .iter()
            .map(|t| {
                let r = LLMChatRecord {
                    id: t.try_get("id")?,
                    insert_time: t.try_get("insert_time")?,
                    session_id: t.try_get("session_id")?,
                    pre_record_id: t.try_get("pre_record_id")?,
                    content: t.try_get("content")?,
                    role: t.try_get("role")?,
                    role_id: t.try_get("role_id")?,
                };
                Ok(r)
            })
            .collect();

        Ok(LLMChatSessionDetailRsp { records: records? })
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<LLMChatDeleteBotReq>,
    ) -> AResult<LLMChatDeleteBotRsp> {
        let updater = SqlUpdater::new("llm_chat_bot")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.bot_id))
            .build(PlaceHolderType::dollar_number())
            .context("unable to build delete template")?;

        self.client()
            .await?
            .execute(&updater.seg, to_sql!(updater.values))
            .await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        let updater = SqlUpdater::new("llm_chat_template")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.template_id))
            .build(PlaceHolderType::dollar_number())
            .context("unable to build delete template")?;

        self.client()
            .await?
            .execute(&updater.seg, to_sql!(updater.values))
            .await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = SqlUpdater::new("llm_chat_session")
            .set("delete_time", Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.session_id))
            .build(PlaceHolderType::dollar_number())
            .context("unable to build delete template")?;

        self.client()
            .await?
            .execute(&updater.seg, to_sql!(updater.values))
            .await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        self.create_table(
            "CREATE TABLE IF NOT EXISTS llm_chat_bot (
                id VARCHAR(40) PRIMARY KEY,
                name VARCHAR(40) NOT NULL,
                body TEXT NOT NULL,
                delete_time TIMESTAMPTZ,
                update_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL,
                svg_logo TEXT NULL
            )",
        )
        .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        self.create_table(
            "CREATE TABLE IF NOT EXISTS llm_chat_template (
                id VARCHAR(40) PRIMARY KEY,
                name VARCHAR(40) NOT NULL,
                prompt TEXT NOT NULL,
                icon_name VARCHAR(200),
                delete_time TIMESTAMPTZ,
                update_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL,
                svg_logo TEXT NULL
            )",
        )
        .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        self.create_table(
            "CREATE TABLE IF NOT EXISTS llm_chat_session (
                id VARCHAR(40) PRIMARY KEY,
                bot_id VARCHAR(40) NOT NULL REFERENCES llm_chat_bot(id),
                template_id VARCHAR(40) NOT NULL REFERENCES llm_chat_template(id),
                title VARCHAR(300) NOT NULL,
                namespace VARCHAR(40) NOT NULL,
                delete_time TIMESTAMPTZ,
                update_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL
            )",
        )
        .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        self.create_table(
            "CREATE TABLE IF NOT EXISTS llm_chat_record (
                id VARCHAR(40) PRIMARY KEY,
                session_id VARCHAR(40) NOT NULL REFERENCES llm_chat_session(id),
                pre_record_id VARCHAR(40),
                content TEXT NOT NULL,
                omit_time TIMESTAMPTZ,
                role VARCHAR(40) NOT NULL,
                role_id VARCHAR(40),
                insert_time TIMESTAMPTZ NOT NULL
            )",
        )
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
            .r#where(Wheres::and([Wheres::equal("id", req.session_id.as_str())]))
            .build(PlaceHolderType::DollarNumber(0))
            .context("unable to build sql")?;

        self.client()
            .await?
            .execute(&updater.seg, to_sql!(updater.values))
            .await?;

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
            if r.id == req.record_id_included {
                remove_flag = true;
            }
            if remove_flag {
                to_omit_ids.push(r.id);
            }
        }

        let updater = SqlUpdater::new("llm_chat_record")
            .set("omit_time", Local::now().fixed_offset())
            .r#where(Wheres::r#in("id", to_omit_ids))
            .build(PlaceHolderType::DollarNumber(0))
            .context("unable to build seg")?;

        self.client()
            .await?
            .execute(&updater.seg, to_sql!(updater.values))
            .await?;

        Ok(LLMChatTruncateSessionRsp {})
    }
}
