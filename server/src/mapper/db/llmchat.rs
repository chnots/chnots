use std::collections::HashMap;

use chin_sql::{ChinSqlError, SqlInserter, SqlReader, SqlUpdater, Wheres};
use chin_tools::{utils::sort_util::sort_by_prev, AResult, EResult};
use chrono::Local;

use super::{KDb, KDbBehaiver, KDbConnBehaiver, KDbRow};
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
        let inserter = SqlInserter::new(LLMChatBot::TABLE)
            .fields(LLMChatBot::ID, &bot.id)
            .fields(LLMChatBot::NAME, &bot.name)
            .fields(LLMChatBot::BODY, &bot.body)
            .fields(LLMChatBot::SVG_LOGO, bot.svg_logo.as_ref())
            .fields(LLMChatBot::INSERT_TIME, &bot.insert_time)
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
            .fields(LLMChatRecord::PRE_RECORD_ID, rec.pre_record_id.as_ref())
            .fields(LLMChatRecord::CONTENT, &rec.content)
            .fields(LLMChatRecord::ROLE, &rec.role)
            .fields(LLMChatRecord::ROLE_ID, rec.role_id.as_ref())
            .fields(LLMChatRecord::REASONING_CONTENT, &rec.reasoning_content)
            .fields(LLMChatRecord::INSERT_TIME, &rec.insert_time);

        self.conn().await?.exec(inserter).await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
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
        req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let query = SqlReader::read_all(LLMChatTemplate::TABLE)
            .r#where(Wheres::and([Wheres::is_null(LLMChatTemplate::DELETE_TIME)]))
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
                Wheres::equal(LLMChatSession::NAMESPACE, &req.namespace),
                Wheres::if_some(req.session_id.as_ref(), |id| {
                    Wheres::equal(LLMChatSession::ID, id)
                }),
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
        let session = self
            .llm_chat_list_sessions(KReq {
                body: LLMChatListSessionReq {
                    session_id: Some(req.session_id.clone()),
                },
                namespace: req.namespace.clone(),
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
            .raw("order by insert_time desc");

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
        let records = self
            .llm_chat_session_detail(KReq {
                body: LLMChatSessionDetialReq {
                    session_id: req.session_id.clone(),
                    with_omit: Some(true),
                },
                namespace: req.namespace.clone(),
            })
            .await?
            .records;

        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for record in &records {
            if let Some(prev) = record.pre_record_id.as_ref() {
                map.entry(prev).or_insert(vec![]).push(&record.id);
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
}
