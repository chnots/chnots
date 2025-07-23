use std::collections::HashMap;

use anyhow::Ok;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{SqlBuilder, Wheres};
use chin_tools::{AResult, EResult};
use itertools::Itertools;

use crate::mapper::db::helper::create_tables;
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTransactionBehaiver
};
use crate::model::dto::KReq;

use super::mapper::LLMChatMapper;
use super::*;

impl TryFrom<KDbRow> for LLMChatRecord {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatRecord {
            otid: value.try_get(LLMChatRecord::OTID)?,
            session_otid: value.try_get(LLMChatRecord::SESSION_OTID)?,
            pre_record_otid: value.try_get(LLMChatRecord::PRE_RECORD_OTID)?,
            content: value.try_get(LLMChatRecord::CONTENT)?,
            role: value.try_get(LLMChatRecord::ROLE)?,
            role_id: value.try_get(LLMChatRecord::ROLE_ID)?,
            reasoning_content: value.try_get(LLMChatRecord::REASONING_CONTENT)?,
            tid: value.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }
}

impl TryFrom<KDbRow> for LLMChatBot {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatBot {
            otid: value.try_get(LLMChatBot::OTID)?,
            name: value.try_get(LLMChatBot::NAME)?,
            body: value.try_get(LLMChatBot::BODY)?,
            update_time: value.try_get(LLMChatBot::UPDATE_TIME)?,
            svg_logo: value.try_get(LLMChatBot::SVG_LOGO)?,
            tid: value.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }
}

impl TryFrom<KDbRow> for LLMChatTemplate {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatTemplate {
            otid: value.try_get(LLMChatTemplate::OTID)?,
            update_time: value.try_get(LLMChatTemplate::UPDATE_TIME)?,
            name: value.try_get(LLMChatTemplate::NAME)?,
            prompt: value.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: value.try_get(LLMChatTemplate::SVG_LOGO)?,
            tid: value.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }
}

impl TryFrom<KDbRow> for LLMChatSession {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatSession {
            otid: value.try_get(LLMChatSession::OTID)?,
            template_otid: value.try_get(LLMChatSession::TEMPLATE_OTID)?,
            title: value.try_get(LLMChatSession::TITLE)?,
            update_time: value.try_get(LLMChatSession::UPDATE_TIME)?,
            tid: value.try_get(LLMChatBot::TID)?,
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
        let otid = bot.otid;

        let inserter = bot.to_sql_inserter();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows(
                LLMChatBot::TABLE,
                &LLMChatBot::create_sql().all_fields(),
                LLMChatBot::pkey_cond(otid),
            )
            .await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatOverwriteBotRsp {})
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp> {
        let tmpl = req.body.template;
        let inserter = tmpl.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows(
                LLMChatTemplate::TABLE,
                &LLMChatTemplate::create_sql().all_fields(),
                LLMChatTemplate::pkey_cond(tmpl.otid),
            )
            .await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatOverwriteTemplateRsp {})
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp> {
        let mut obj = req.body.session;
        let s: String = obj.title.to_string().chars().take(199).collect();
        obj.title = s.try_into()?;
        let inserter = obj.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows(
                LLMChatSession::TABLE,
                &LLMChatSession::create_sql().all_fields(),
                LLMChatSession::pkey_cond(obj.otid),
            )
            .await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatInsertSessionRsp {})
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp> {
        let obj = req.body.record;
        let otid = obj.otid;
        let inserter = obj.to_owned().to_sql_inserter();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows(
                LLMChatRecord::TABLE,
                &LLMChatRecord::create_sql().all_fields(),
                LLMChatRecord::pkey_cond(otid),
            )
            .await?;
        tx.exec(inserter).await?;
        tx.cmt().await?;

        Ok(LLMChatInsertRecordRsp {})
    }

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp> {
        let _ = req;
        let sql = format!(
            "select * from {} bot left join (select {}, count({}) as bcount from {} where {} > {} group by {}) rc on bot.{} = rc.{} order by bcount desc",
            LLMChatBot::TABLE,
            LLMChatRecord::ROLE_ID,
            LLMChatRecord::ROLE_ID,
            LLMChatRecord::TABLE,
            LLMChatBot::TID,
            TID::default().as_num() - 14 * 24 * 3600 * 1000000,
            LLMChatRecord::ROLE_ID,
            LLMChatBot::OTID,
            LLMChatRecord::ROLE_ID
        );

        let bots = self
            .conn()
            .await?
            .qry_list(sql, |row| {
                let c: Option<i64> = row.try_get("bcount")?;

                Ok((c.unwrap_or(0), LLMChatBot::try_from(row)?))
            })
            .await?
            .into_iter()
            .sorted_by(|r1, r2| r2.0.cmp(&r1.0))
            .map(|(_, bot)| bot)
            .collect();

        Ok(LLMChatListBotRsp { bots })
    }

    async fn llm_chat_list_templates(
        &self,
        _req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp> {
        let sql = format!(
            "select b.*, count(r.{}) as bot_count from {} b left join {} r on b.{} = r.{} group by b.{} order by bot_count desc",
            LLMChatRecord::ROLE_ID,
            LLMChatTemplate::TABLE,
            LLMChatRecord::TABLE,
            LLMChatTemplate::OTID,
            LLMChatRecord::ROLE_ID,
            LLMChatTemplate::OTID,
        );

        let templates: Vec<LLMChatTemplate> = self
            .conn()
            .await?
            .qry_list(sql, LLMChatTemplate::try_from)
            .await?;

        Ok(LLMChatListTemplateRsp { templates })
    }

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<LLMChatListSessionReq>,
    ) -> AResult<LLMChatListSessionRsp> {
        let query = SqlBuilder::read_all(LLMChatSession::TABLE)
            .r#where(Wheres::and([Wheres::if_some(
                req.session_otid.as_ref(),
                |tid| Wheres::equal(LLMChatSession::OTID, *tid),
            )]))
            .sov("order by tid desc");

        let sessions = self
            .conn()
            .await?
            .qry_list(query, LLMChatSession::try_from)
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
            .r#where(Wheres::and([Wheres::equal(
                LLMChatRecord::SESSION_OTID,
                req.session_otid,
            )]))
            .sov("order by tid desc");

        let records: Vec<LLMChatRecord> = self
            .conn()
            .await?
            .qry_list(query, LLMChatRecord::try_from)
            .await?;

        Ok(LLMChatSessionDetailRsp { session, records })
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<LLMChatDeleteBotReq>,
    ) -> AResult<LLMChatDeleteBotRsp> {
        self.conn()
            .await?
            .as_executor()
            .omit_rows(
                LLMChatBot::TABLE,
                &LLMChatBot::create_sql().all_fields(),
                LLMChatBot::pkey_cond(req.bot_otid),
            )
            .await?;

        Ok(LLMChatDeleteBotRsp {})
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp> {
        self.conn()
            .await?
            .as_executor()
            .omit_rows(
                LLMChatTemplate::TABLE,
                &LLMChatTemplate::create_sql().all_fields(),
                LLMChatTemplate::pkey_cond(req.template_otid),
            )
            .await?;

        Ok(LLMChatDeleteTemplateRsp {})
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp> {
        self.conn()
            .await?
            .as_executor()
            .omit_rows(
                LLMChatSession::TABLE,
                &LLMChatSession::create_sql().all_fields(),
                LLMChatSession::pkey_cond(req.session_otid),
            )
            .await?;

        Ok(LLMChatDeleteSessionRsp {})
    }

    async fn ensure_table_llm_chat(&self) -> EResult {
        create_tables(
            vec![
                LLMChatBot::create_sql().to_owned_sql(),
                LLMChatBot::hist_table(),
                LLMChatTemplate::create_sql().to_owned_sql(),
                LLMChatTemplate::hist_table(),
                LLMChatSession::create_sql().to_owned_sql(),
                LLMChatSession::hist_table(),
                LLMChatRecord::create_sql().to_owned_sql(),
                LLMChatRecord::hist_table(),
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

        let pk_read = LLMChatSession::pkey_reader(req.session_otid);
        let mut sess = tx.qry_one(pk_read, LLMChatSession::try_from, false).await?;

        tx.as_executor()
            .omit_rows(
                LLMChatSession::TABLE,
                &LLMChatSession::create_sql().all_fields(),
                LLMChatSession::pkey_cond(req.session_otid),
            )
            .await?;

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

        self.conn()
            .await?
            .as_executor()
            .omit_rows(
                LLMChatRecord::TABLE,
                &LLMChatRecord::create_sql().all_fields(),
                Wheres::and([Wheres::r#in(LLMChatRecord::OTID, to_omit_ids)]),
            )
            .await?;

        Ok(LLMChatTruncateSessionRsp { count: 0 })
    }
}
