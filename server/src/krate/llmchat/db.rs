use std::collections::HashMap;

use anyhow::Ok;
use chin_sql::{SqlInserter, SqlReader, SqlUpdater, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;

use crate::mapper::db::tabledumpsql::TableDumpSqlBuilder;
use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier};
use crate::model::dto::KReq;

use super::mapper::{LLMChatDeserializeMapper, LLMChatDumpMapper, LLMChatMapper};
use super::*;

impl LLMChatDeserializeMapper for KDbRow {
    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: self.try_get(LLMChatBot::ID)?,
            insert_time: self.try_get(LLMChatBot::INSERT_TIME)?,
            delete_time: self.try_get(LLMChatBot::DELETE_TIME)?,
            name: self.try_get(LLMChatBot::NAME)?,
            body: self.try_get(LLMChatBot::BODY)?,
            update_time: self.try_get(LLMChatBot::UPDATE_TIME)?,
            svg_logo: self.try_get(LLMChatBot::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: self.try_get(LLMChatTemplate::ID)?,
            insert_time: self.try_get(LLMChatTemplate::INSERT_TIME)?,
            delete_time: self.try_get(LLMChatTemplate::DELETE_TIME)?,
            update_time: self.try_get(LLMChatTemplate::UPDATE_TIME)?,
            name: self.try_get(LLMChatTemplate::NAME)?,
            prompt: self.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: self.try_get(LLMChatTemplate::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: self.try_get(LLMChatSession::ID)?,
            insert_time: self.try_get(LLMChatSession::INSERT_TIME)?,
            template_id: self.try_get(LLMChatSession::TEMPLATE_ID)?,
            title: self.try_get(LLMChatSession::TITLE)?,
            kspace: self.try_get(LLMChatSession::KSPACE)?,
            delete_time: self.try_get(LLMChatSession::DELETE_TIME)?,
            update_time: self.try_get(LLMChatSession::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: self.try_get(LLMChatRecord::ID)?,
            insert_time: self.try_get(LLMChatRecord::INSERT_TIME)?,
            session_id: self.try_get(LLMChatRecord::SESSION_ID)?,
            pre_record_id: self.try_get(LLMChatRecord::PRE_RECORD_ID)?,
            content: self.try_get(LLMChatRecord::CONTENT)?,
            role: self.try_get(LLMChatRecord::ROLE)?,
            role_id: self.try_get(LLMChatRecord::ROLE_ID)?,
            omit_time: self.try_get(LLMChatRecord::OMIT_TIME)?,
            reasoning_content: self.try_get(LLMChatRecord::REASONING_CONTENT)?,
        };
        Ok(obj)
    }
}

impl LLMChatMapper for KDb {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<LLMChatOverwriteBotReq>,
    ) -> AResult<LLMChatOverwriteBotRsp> {
        let bot = &req.bot;
        let inserter = SqlInserter::new(LLMChatBot::TABLE)
            .field(LLMChatBot::ID, &bot.id)
            .field(LLMChatBot::NAME, &bot.name)
            .field(LLMChatBot::BODY, &bot.body)
            .field(LLMChatBot::SVG_LOGO, bot.svg_logo.as_ref())
            .field(LLMChatBot::INSERT_TIME, bot.insert_time)
            .on_conflict(chin_sql::OnConflict::Replace("id".to_string()));

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        let tmpl = &req.template;
        let inserter = SqlInserter::new(LLMChatTemplate::TABLE)
            .field(LLMChatTemplate::ID, &tmpl.id)
            .field(LLMChatTemplate::NAME, &tmpl.name)
            .field(LLMChatTemplate::PROMPT, &tmpl.prompt)
            .field(LLMChatTemplate::SVG_LOGO, tmpl.svg_logo.as_ref())
            .field(LLMChatTemplate::INSERT_TIME, tmpl.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let title: String = req.session.title.chars().take(300).collect();
        let session = &req.session;
        let inserter = SqlInserter::new(LLMChatSession::TABLE)
            .field(LLMChatSession::ID, &session.id)
            .field(LLMChatSession::TEMPLATE_ID, &session.template_id)
            .field(LLMChatSession::TITLE, &title)
            .field(LLMChatSession::KSPACE, &session.kspace)
            .field(LLMChatSession::INSERT_TIME, session.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        let rec = &req.record;
        let inserter = SqlInserter::new(LLMChatRecord::TABLE)
            .field(LLMChatRecord::ID, &rec.id)
            .field(LLMChatRecord::SESSION_ID, &rec.session_id)
            .field(LLMChatRecord::PRE_RECORD_ID, rec.pre_record_id.as_ref())
            .field(LLMChatRecord::CONTENT, &rec.content)
            .field(LLMChatRecord::ROLE, &rec.role)
            .field(LLMChatRecord::ROLE_ID, rec.role_id.as_ref())
            .field(LLMChatRecord::REASONING_CONTENT, &rec.reasoning_content)
            .field(LLMChatRecord::INSERT_TIME, rec.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let _ = req;
        let sql = "select b.*, count(r.role_id) as bot_count from llm_chat_bot b left join llm_chat_record r on b.id = r.role_id where b.delete_time is null group by b.id order by bot_count desc";
        let bots = self
            .conn()
            .await?
            .qry_list(sql, |e| KDbRow::to_llmchat_bot(e))
            .await?
            .into_iter()
            .collect();

        Ok(LLMChatListBotRsp { bots })
    }

    async fn llm_chat_list_templates(
        &self,
        _req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let query = SqlReader::read_all(LLMChatTemplate::TABLE)
            .r#where(Wheres::and([Wheres::is_null(LLMChatTemplate::DELETE_TIME)]))
            .sov("order by insert_time desc");

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
                Wheres::equal(LLMChatSession::KSPACE, &req.kspace),
                Wheres::if_some(req.session_id.as_ref(), |id| {
                    Wheres::equal(LLMChatSession::ID, id)
                }),
            ]))
            .sov("order by insert_time desc");

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
        let session = self
            .llm_chat_list_sessions(KReq {
                body: LLMChatListSessionReq {
                    session_id: Some(req.session_id.clone()),
                },
                kspace: req.kspace.clone(),
                mkspaces: vec![],
            })
            .await?
            .sessions
            .into_iter()
            .nth(0);

        let query = SqlReader::read_all(LLMChatRecord::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(LLMChatRecord::SESSION_ID, req.session_id.clone()),
                Wheres::if_some(
                    {
                        match req.with_omit.as_ref() {
                            Some(flag) => match flag {
                                true => None,
                                false => Some(()),
                            },
                            None => Some(()),
                        }
                    },
                    |_| Wheres::is_null(LLMChatRecord::OMIT_TIME),
                ),
            ]))
            .sov("order by insert_time desc");

        let records: Vec<LLMChatRecord> = self
            .conn()
            .await?
            .qry_list(query, |e| e.to_llmchat_record())
            .await?;

        Ok(LLMChatSessionDetailRsp { session, records })
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
            .set(LLMChatTemplate::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::equal("id", &req.template_id));

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = SqlUpdater::new(LLMChatSession::TABLE)
            .set(LLMChatSession::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::equal(LLMChatSession::ID, &req.session_id));
        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        self.conn()
            .await?
            .exec(LLMChatBot::schema(self.conn().await?.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        self.conn()
            .await?
            .create_table(LLMChatTemplate::schema(self.conn().await?.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        self.conn()
            .await?
            .create_table(LLMChatSession::schema(self.conn().await?.db_type()))
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        self.conn()
            .await?
            .create_table(LLMChatRecord::schema(self.conn().await?.db_type()))
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
        let records = self
            .llm_chat_session_detail(KReq {
                body: LLMChatSessionDetialReq {
                    session_id: req.session_id.clone(),
                    with_omit: Some(true),
                },
                kspace: req.kspace.clone(),
                mkspaces: vec![],
            })
            .await?
            .records;

        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for record in &records {
            if let Some(prev) = record.pre_record_id.as_ref() {
                map.entry(prev).or_default().push(&record.id);
            }
        }

        let mut to_omit_ids: Vec<&str> = vec![req.remove_rid_included.as_ref()];
        let vec = vec![];
        let mut queue: Vec<&str> = map
            .get(req.remove_rid_included.as_str())
            .unwrap_or(&vec)
            .to_vec();

        loop {
            if queue.is_empty() {
                break;
            }
            let mut tmp = vec![];

            for r in queue {
                to_omit_ids.push(r);
                if let Some(v) = map.get(r) {
                    tmp.extend(v);
                }
            }
            queue = tmp;
        }

        let updater = SqlUpdater::new(LLMChatRecord::TABLE)
            .set(LLMChatRecord::OMIT_TIME, Local::now().fixed_offset())
            .r#where(Wheres::and([
                Wheres::r#in(LLMChatRecord::ID, to_omit_ids),
                Wheres::is_null(LLMChatRecord::OMIT_TIME),
            ]));

        let count = self.conn().await?.exec(updater).await?;

        Ok(LLMChatTruncateSessionRsp { count })
    }

    async fn ensure_table_llm_chat(&self) -> EResult {
        self.ensure_table_llm_chat_bot().await?;
        self.ensure_table_llm_chat_template().await?;
        self.ensure_table_llm_chat_session().await?;
        self.ensure_table_llm_chat_record().await?;

        Ok(())
    }
}

impl LLMChatDumpMapper for KDb {
    async fn dump_llmchat_bot(
        &self,
        callback: &crate::mapper::dump::RecordCallbackType,
    ) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(LLMChatBot::TABLE),
            KDbRow::to_llmchat_bot,
            callback,
        )
        .await?;

        Ok(())
    }

    async fn dump_llmchat_template(
        &self,
        callback: &crate::mapper::dump::RecordCallbackType,
    ) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(LLMChatRecord::TABLE),
            KDbRow::to_llmchat_record,
            callback,
        )
        .await?;

        Ok(())
    }

    async fn dump_llmchat_session(
        &self,
        callback: &crate::mapper::dump::RecordCallbackType,
    ) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(LLMChatSession::TABLE),
            KDbRow::to_llmchat_session,
            callback,
        )
        .await?;

        Ok(())
    }

    async fn dump_llmchat_record(
        &self,
        callback: &crate::mapper::dump::RecordCallbackType,
    ) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(LLMChatTemplate::TABLE),
            KDbRow::to_llmchat_template,
            callback,
        )
        .await?;

        Ok(())
    }
}
