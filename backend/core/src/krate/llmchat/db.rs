use std::collections::HashMap;

use anyhow::Ok;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{SqlBuilder, SqlUpdater, Wheres};
use chin_tools::{AResult, EResult};

use crate::mapper::db::helper::create_tables;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;

use super::mapper::{LLMChatDeserializeMapper, LLMChatMapper};
use super::*;

impl LLMChatDeserializeMapper for KDbRow {
    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            otid: self.try_get(LLMChatBot::OTID)?,
            omit_tid: self.try_get(LLMChatBot::OMIT_TID)?,
            name: self.try_get(LLMChatBot::NAME)?,
            body: self.try_get(LLMChatBot::BODY)?,
            update_time: self.try_get(LLMChatBot::UPDATE_TIME)?,
            svg_logo: self.try_get(LLMChatBot::SVG_LOGO)?,
            tid: self.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            otid: self.try_get(LLMChatTemplate::OTID)?,
            omit_tid: self.try_get(LLMChatTemplate::OMIT_TID)?,
            update_time: self.try_get(LLMChatTemplate::UPDATE_TIME)?,
            name: self.try_get(LLMChatTemplate::NAME)?,
            prompt: self.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: self.try_get(LLMChatTemplate::SVG_LOGO)?,
            tid: self.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            otid: self.try_get(LLMChatSession::OTID)?,
            template_otid: self.try_get(LLMChatSession::TEMPLATE_OTID)?,
            title: self.try_get(LLMChatSession::TITLE)?,
            omit_tid: self.try_get(LLMChatSession::OMIT_TID)?,
            update_time: self.try_get(LLMChatSession::UPDATE_TIME)?,
            tid: self.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            otid: self.try_get(LLMChatRecord::OTID)?,
            session_otid: self.try_get(LLMChatRecord::SESSION_OTID)?,
            pre_record_otid: self.try_get(LLMChatRecord::PRE_RECORD_OTID)?,
            content: self.try_get(LLMChatRecord::CONTENT)?,
            role: self.try_get(LLMChatRecord::ROLE)?,
            role_id: self.try_get(LLMChatRecord::ROLE_ID)?,
            omit_tid: self.try_get(LLMChatRecord::OMIT_TID)?,
            reasoning_content: self.try_get(LLMChatRecord::REASONING_CONTENT)?,
            tid: self.try_get(LLMChatBot::TID)?,
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

        let omit = LLMChatBot::pkey_updater(bot.otid, OmitTID::never())
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
        let omit = LLMChatTemplate::pkey_updater(tmpl.otid, OmitTID::never())
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
        let omit = LLMChatSession::pkey_updater(obj.otid, OmitTID::never())
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
            LLMChatRecord::pkey_updater(obj.otid).set(LLMChatRecord::OMIT_TID, OmitTID::now());
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
        let sql = format!(
            "select b.*, count(r.{}) as bot_count from {} b left join {} r on b.{} = r.{} where b.{} = {} group by b.{}, b.{} order by bot_count desc",
            LLMChatRecord::ROLE_ID,
            LLMChatBot::TABLE,
            LLMChatRecord::TABLE,
            LLMChatBot::OTID,
            LLMChatRecord::ROLE_ID,
            LLMChatBot::OMIT_TID,
            OmitTID::never().as_num(),
            LLMChatBot::OTID,
            LLMChatBot::OMIT_TID
        );

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
        let sql = format!(
            "select b.*, count(r.{}) as bot_count from {} b left join {} r on b.{} = r.{} where b.{} = {} group by b.{}, b.{} order by bot_count desc",
            LLMChatRecord::ROLE_ID,
            LLMChatTemplate::TABLE,
            LLMChatRecord::TABLE,
            LLMChatTemplate::OTID,
            LLMChatRecord::ROLE_ID,
            LLMChatTemplate::OMIT_TID,
            OmitTID::never().as_num(),
            LLMChatTemplate::OTID,
            LLMChatTemplate::OMIT_TID
        );

        let templates: Vec<LLMChatTemplate> = self
            .conn()
            .await?
            .qry_list(sql, |e| e.to_llmchat_template())
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
                Wheres::if_some(req.session_otid.as_ref(), |tid| {
                    Wheres::equal(LLMChatSession::OTID, *tid)
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
                    session_otid: Some(req.session_otid),
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
                Wheres::equal(LLMChatRecord::SESSION_OTID, req.session_otid),
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
        let updater = LLMChatBot::pkey_updater(req.bot_otid, OmitTID::never())
            .set(LLMChatBot::OMIT_TID, OmitTID::now());

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        let updater = LLMChatTemplate::pkey_updater(req.template_otid, OmitTID::never())
            .set(LLMChatTemplate::OMIT_TID, OmitTID::now());

        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        let updater = LLMChatSession::pkey_updater(req.session_otid, OmitTID::never())
            .set(LLMChatSession::OMIT_TID, OmitTID::now());
        self.conn().await?.exec(updater).await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat(&self) -> EResult {
        create_tables(
            vec![
                LLMChatBot::create_sql(),
                LLMChatTemplate::create_sql(),
                LLMChatSession::create_sql(),
                LLMChatRecord::create_sql(),
            ],
            self,
        )
        .await
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        let pk_read = LLMChatSession::pkey_reader(req.session_otid, OmitTID::never());
        let mut sess = tx
            .qry_one(pk_read, KDbRow::to_llmchat_session, false)
            .await?;

        let pk_update = LLMChatSession::pkey_updater(req.session_otid, OmitTID::never())
            .set(LLMChatSession::OMIT_TID, OmitTID::now());
        tx.exec(pk_update).await?;

        sess.omit_tid = OmitTID::never();
        if let Some(title) = req.body.title {
            sess.title = Varchar::<500>::limit(title);
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
                    session_otid: req.session_otid,
                    with_omit: Some(true),
                },
                kspace: req.kspace.clone(),
                mkspaces: vec![],
            })
            .await?
            .records;

        let mut map: HashMap<TID, Vec<TID>> = HashMap::new();
        for record in &records {
            if let Some(prev) = record.pre_record_otid {
                map.entry(prev).or_default().push(record.otid);
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
                Wheres::r#in(LLMChatRecord::OTID, to_omit_ids),
                Wheres::equal(LLMChatRecord::OMIT_TID, OmitTID::never()),
            ]));

        let count = self.conn().await?.exec(updater).await?;

        Ok(LLMChatTruncateSessionRsp { count })
    }
}