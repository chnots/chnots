use std::collections::HashMap;

use anyhow::Ok;
use chin_sql::time_type::TID;
use chin_sql::{SqlBuilder, SqlUpdater, Wheres};
use chin_tools::{AResult, EResult};

use crate::mapper::db::tabledumpsql::TableDumpSqlBuilder;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;

use super::mapper::{LLMChatDeserializeMapper, LLMChatDumpMapper, LLMChatMapper};
use super::*;

impl LLMChatDeserializeMapper for KDbRow {
    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            tid: self.try_get(LLMChatBot::TID)?,
            omit_tid: self.try_get(LLMChatBot::OMIT_TID)?,
            name: self.try_get(LLMChatBot::NAME)?,
            body: self.try_get(LLMChatBot::BODY)?,
            update_time: self.try_get(LLMChatBot::UPDATE_TIME)?,
            svg_logo: self.try_get(LLMChatBot::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            tid: self.try_get(LLMChatTemplate::TID)?,
            omit_tid: self.try_get(LLMChatTemplate::OMIT_TID)?,
            update_time: self.try_get(LLMChatTemplate::UPDATE_TIME)?,
            name: self.try_get(LLMChatTemplate::NAME)?,
            prompt: self.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: self.try_get(LLMChatTemplate::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            tid: self.try_get(LLMChatSession::TID)?,
            template_tid: self.try_get(LLMChatSession::TEMPLATE_TID)?,
            title: self.try_get(LLMChatSession::TITLE)?,
            omit_tid: self.try_get(LLMChatSession::OMIT_TID)?,
            update_time: self.try_get(LLMChatSession::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            tid: self.try_get(LLMChatRecord::TID)?,
            session_tid: self.try_get(LLMChatRecord::SESSION_TID)?,
            pre_record_tid: self.try_get(LLMChatRecord::PRE_RECORD_TID)?,
            content: self.try_get(LLMChatRecord::CONTENT)?,
            role: self.try_get(LLMChatRecord::ROLE)?,
            role_id: self.try_get(LLMChatRecord::ROLE_ID)?,
            omit_tid: self.try_get(LLMChatRecord::OMIT_TID)?,
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
        let bot = req.body.bot;

        let omit = LLMChatBot::pkey_updater(bot.tid, OmitTID::never())
            .set(LLMChatBot::OMIT_TID, OmitTID::now());
        let inserter = bot.to_sql_inserter();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.exec(omit).await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        let tmpl = req.body.template;
        let omit = LLMChatTemplate::pkey_updater(tmpl.tid, OmitTID::never())
            .set(LLMChatTemplate::OMIT_TID, OmitTID::now());
        let inserter = tmpl.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.exec(omit).await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let obj = req.body.session;
        let omit = LLMChatSession::pkey_updater(obj.tid, OmitTID::never())
            .set(LLMChatSession::OMIT_TID, OmitTID::now());
        let inserter = obj.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.exec(omit).await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        let obj = req.body.record;
        let omit =
            LLMChatRecord::pkey_updater(obj.tid).set(LLMChatRecord::OMIT_TID, OmitTID::now());
        let inserter = obj.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.exec(omit).await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let _ = req;
        let sql = "select b.*, count(r.role_id) as bot_count from llm_chat_bot b left join llm_chat_record r on b.tid = r.role_id where b.omit_tid = ".to_string() + &format!("{}", OmitTID::never().as_num()) + " group by b.tid, b.omit_tid order by bot_count desc";
        let bots = self
            .conn()
            .await?
            .qry_list(sql, KDbRow::to_llmchat_bot)
            .await?
            .into_iter()
            .collect();

        Ok(LLMChatListBotRsp { bots })
    }

    async fn llm_chat_list_templates(
        &self,
        _req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let query = SqlBuilder::read_all(LLMChatTemplate::TABLE)
            .r#where(Wheres::and([Wheres::equal(
                LLMChatTemplate::OMIT_TID,
                OmitTID::never(),
            )]))
            .sov("order by tid desc");

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
        let query = SqlBuilder::read_all(LLMChatSession::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(LLMChatSession::OMIT_TID, OmitTID::never()),
                Wheres::if_some(req.session_tid.as_ref(), |tid| {
                    Wheres::equal(LLMChatSession::TID, *tid)
                }),
            ]))
            .sov("order by tid desc");

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
                    session_tid: Some(req.session_tid),
                },
                kspace: req.kspace.clone(),
                mkspaces: vec![],
            })
            .await?
            .sessions
            .into_iter()
            .nth(0);

        let query = SqlBuilder::read_all(LLMChatRecord::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(LLMChatRecord::SESSION_TID, req.session_tid),
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
                    |_| Wheres::equal(LLMChatRecord::OMIT_TID, OmitTID::never()),
                ),
            ]))
            .sov("order by tid desc");

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
        let updater = LLMChatBot::pkey_updater(req.bot_tid, OmitTID::never())
            .set(LLMChatBot::OMIT_TID, OmitTID::now());

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        let updater = LLMChatTemplate::pkey_updater(req.template_tid, OmitTID::never())
            .set(LLMChatTemplate::OMIT_TID, OmitTID::now());

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = LLMChatSession::pkey_updater(req.session_tid, OmitTID::never())
            .set(LLMChatSession::OMIT_TID, OmitTID::now());
        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        self.conn().await?.exec(LLMChatBot::create_sql()).await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        self.conn()
            .await?
            .exec(LLMChatTemplate::create_sql())
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        self.conn()
            .await?
            .exec(LLMChatSession::create_sql())
            .await?;
        Ok(())
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        self.conn().await?.exec(LLMChatRecord::create_sql()).await?;
        Ok(())
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        let pk_read = LLMChatSession::pkey_reader(req.session_tid, OmitTID::never());
        let mut sess = tx
            .qry_one(pk_read, KDbRow::to_llmchat_session, false)
            .await?;

        let pk_update = LLMChatSession::pkey_updater(req.session_tid, OmitTID::never())
            .set(LLMChatSession::OMIT_TID, OmitTID::now());
        tx.exec(pk_update).await?;

        sess.omit_tid = OmitTID::never();
        if let Some(title) = req.body.title {
            sess.title = title;
        }
        if let Some(true) = req.body.delete {
            return Ok(LLMChatUpdateSessionRsp {});
        }
        tx.exec(sess.to_sql_inserter()).await?;

        tx.cmt().await?;

        Ok(LLMChatUpdateSessionRsp {})
    }

    async fn llm_chat_truncate_session(
        &self,
        req: KReq<LLMChatTruncateSessionReq>,
    ) -> AResult<LLMChatTruncateSessionRsp> {
        let records = self
            .llm_chat_session_detail(KReq {
                body: LLMChatSessionDetialReq {
                    session_tid: req.session_tid,
                    with_omit: Some(true),
                },
                kspace: req.kspace.clone(),
                mkspaces: vec![],
            })
            .await?
            .records;

        let mut map: HashMap<TID, Vec<TID>> = HashMap::new();
        for record in &records {
            if let Some(prev) = record.pre_record_tid {
                map.entry(prev).or_default().push(record.tid);
            }
        }

        let mut to_omit_ids: Vec<TID> = vec![req.remove_rid_included];
        let vec = vec![];
        let mut queue: Vec<TID> = map.get(&req.remove_rid_included).unwrap_or(&vec).to_vec();

        loop {
            if queue.is_empty() {
                break;
            }
            let mut tmp = vec![];

            for r in queue {
                to_omit_ids.push(r);
                if let Some(v) = map.get(&r) {
                    tmp.extend(v);
                }
            }
            queue = tmp;
        }

        let updater = SqlUpdater::new(LLMChatRecord::TABLE)
            .set(LLMChatRecord::OMIT_TID, OmitTID::now())
            .r#where(Wheres::and([
                Wheres::r#in(LLMChatRecord::TID, to_omit_ids),
                Wheres::equal(LLMChatRecord::OMIT_TID, OmitTID::never()),
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
