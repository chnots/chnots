use std::collections::HashMap;

use anyhow::Ok;
use chin_sql::time_type::TID;
use chin_sql::{SqlBuilder, Wheres};
use chin_tools::{AResult, EResult};
use itertools::Itertools;

use crate::mapper::Curd;
use crate::mapper::db::helper::{Ddls, print_ddls};
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::OtidTableSupport;
use crate::model::dto::KReq;

use super::mapper::LLMChatMapper;
use super::*;

impl TryFrom<&KDbRow> for LLMChatRecord {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatRecord {
            otid: value.try_get(LLMChatRecord::OTID)?,
            session_otid: value.try_get(LLMChatRecord::SESSION_OTID)?,
            pre_record_otid: value.try_get(LLMChatRecord::PRE_RECORD_OTID)?,
            content: value.try_get(LLMChatRecord::CONTENT)?,
            role: value.try_get(LLMChatRecord::ROLE)?,
            role_id: value.try_get(LLMChatRecord::ROLE_ID)?,
            tid: value.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }
}

impl TryFrom<&KDbRow> for LLMChatBot {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
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

impl TryFrom<&KDbRow> for LLMChatTemplate {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
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

impl TryFrom<&KDbRow> for LLMChatSession {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = LLMChatSession {
            otid: value.try_get(LLMChatSession::OTID)?,
            template_otid: value.try_get(LLMChatSession::TEMPLATE_OTID)?,
            update_time: value.try_get(LLMChatSession::UPDATE_TIME)?,
            tid: value.try_get(LLMChatBot::TID)?,
        };
        Ok(obj)
    }
}

impl LLMChatMapper for KDb {
    async fn llmchat_bot_commit(
        &self,
        req: KReq<LLMChatBotCommitReq>,
    ) -> AResult<LLMChatBotCommitRsp> {
        self.po_otid_insert([req.body.bot]).await?;

        Ok(LLMChatBotCommitRsp {})
    }

    async fn llmchat_template_commit(
        &self,
        req: KReq<LLMChatTemplateCommitReq>,
    ) -> AResult<LLMChatTemplateCommitRsp> {
        self.po_otid_insert([req.body.template]).await?;

        Ok(LLMChatTemplateCommitRsp {})
    }

    async fn llmchat_session_commit(
        &self,
        req: KReq<LLMChatSessionCommitReq>,
    ) -> AResult<LLMChatSessionCommitRsp> {
        let session = req.body.session;
        self.po_otid_insert([session]).await?;

        Ok(LLMChatSessionCommitRsp {})
    }

    async fn llmchat_record_commit(
        &self,
        req: KReq<LLMChatRecordCommitReq>,
    ) -> AResult<LLMChatRecordCommitRsp> {
        self.po_otid_insert([req.body.record]).await?;

        Ok(LLMChatRecordCommitRsp {})
    }

    async fn llmchat_bot_list(&self, req: KReq<LLMChatBotListReq>) -> AResult<LLMChatBotListRsp> {
        let _ = req;
        let sql = format!(
            "select * from {} bot left join (select {}, count({}) as bcount from {} where {} > {} group by {}) rc on bot.{} = rc.{} order by bcount desc",
            LLMChatBot::TABLE,
            LLMChatRecord::ROLE_ID,
            LLMChatRecord::ROLE_ID,
            LLMChatRecord::TABLE,
            LLMChatBot::TID,
            TID::now().as_num() - 14 * 24 * 3600 * 1000000,
            LLMChatRecord::ROLE_ID,
            LLMChatBot::OTID,
            LLMChatRecord::ROLE_ID
        );

        let bots = self
            .conn()
            .await?
            .qry_list(sql, |row| {
                let c: Option<i64> = row.try_get("bcount")?;

                Ok((c.unwrap_or(0), (&row).try_into()?))
            })
            .await?
            .into_iter()
            .sorted_by(|r1, r2| r2.0.cmp(&r1.0))
            .map(|(_, bot)| bot)
            .collect();

        Ok(LLMChatBotListRsp { bots })
    }

    async fn llmchat_template_list(
        &self,
        _req: KReq<LLMChatTemplateListReq>,
    ) -> AResult<LLMChatTemplateListRsp> {
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
            .qry_list(sql, |r| LLMChatTemplate::try_from(&r))
            .await?;

        Ok(LLMChatTemplateListRsp { templates })
    }

    async fn llmchat_session_list(
        &self,
        req: KReq<LLMChatSessionListReq>,
    ) -> AResult<LLMChatSessionListRsp> {
        let query = SqlBuilder::read_all(LLMChatSession::TABLE)
            .r#where(Wheres::and([Wheres::if_some(
                req.session_otid.as_ref(),
                |tid| Wheres::equal(LLMChatSession::OTID, *tid),
            )]))
            .seg("order by tid desc");

        let sessions = self
            .conn()
            .await?
            .qry_list(query, |r| LLMChatSession::try_from(&r))
            .await?;

        Ok(LLMChatSessionListRsp { sessions })
    }

    async fn llmchat_session_record_fetch(
        &self,
        req: KReq<LLMChatSessionRecordFetchReq>,
    ) -> AResult<LLMChatSessionRecordFetchRsp> {
        let session = self
            .llmchat_session_list(KReq {
                body: LLMChatSessionListReq {
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
            .seg("order by tid desc");

        let mut records: Vec<LLMChatRecord> = self
            .conn()
            .await?
            .qry_list(query, |r| LLMChatRecord::try_from(&r))
            .await?;
        if req.body.include_hist.unwrap_or_default() {
            let query = SqlBuilder::read_all(LLMChatRecord::table_name(true))
                .r#where(Wheres::and([Wheres::equal(
                    LLMChatRecord::SESSION_OTID,
                    req.session_otid,
                )]))
                .seg("order by tid desc");

            let hist_recs: Vec<LLMChatRecord> = self
                .conn()
                .await?
                .qry_list(query, |r| LLMChatRecord::try_from(&r))
                .await?;
            records.extend(hist_recs);
        }

        Ok(LLMChatSessionRecordFetchRsp { session, records })
    }

    async fn llmchat_bot_archive(
        &self,
        req: KReq<LLMChatBotArchiveReq>,
    ) -> AResult<LLMChatBotArchiveRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.omit_rows::<LLMChatBot>(LLMChatBot::pkey_cond(req.bot_otid))
            .await?;

        tx.cmt().await?;
        Ok(LLMChatBotArchiveRsp {})
    }

    async fn llmchat_template_archive(
        &self,
        req: KReq<LLMChatTemplateArchiveReq>,
    ) -> AResult<LLMChatTemplateArchiveRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.omit_rows::<LLMChatTemplate>(LLMChatTemplate::pkey_cond(req.template_otid))
            .await?;

        tx.cmt().await?;
        Ok(LLMChatTemplateArchiveRsp {})
    }

    async fn llmchat_session_archive(
        &self,
        req: KReq<LLMChatSessionArchiveReq>,
    ) -> AResult<LLMChatSessionArchiveRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.omit_rows::<LLMChatSession>(LLMChatSession::pkey_cond(req.session_otid))
            .await?;
        tx.cmt().await?;

        Ok(LLMChatSessionArchiveRsp {})
    }

    async fn ensure_table_llm_chat(&self) -> EResult {
        print_ddls(
            Ddls::new()
                .with_ddls(LLMChatBot::ddls())
                .with_ddls(LLMChatTemplate::ddls())
                .with_ddls(LLMChatSession::ddls())
                .with_ddls(LLMChatRecord::ddls()),
            self,
        )
        .await
    }

    async fn llmchat_session_record_truncate(
        &self,
        req: KReq<LLMChatSessionRecordTruncateReq>,
    ) -> AResult<LLMChatSessionRecordTruncateRsp> {
        let records = self
            .llmchat_session_record_fetch(KReq {
                body: LLMChatSessionRecordFetchReq {
                    session_otid: req.session_otid,
                    include_hist: false.into(),
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

        let mut to_omit_ids: Vec<TID> = vec![req.remove_otid_included];
        let vec = vec![];
        let mut queue: Vec<TID> = map.get(&req.remove_otid_included).unwrap_or(&vec).to_vec();

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

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.omit_rows::<LLMChatRecord>(Wheres::and([Wheres::r#in(
            LLMChatRecord::OTID,
            to_omit_ids,
        )]))
        .await?;
        tx.cmt().await?;

        Ok(LLMChatSessionRecordTruncateRsp { count: 0 })
    }
}

impl Curd for LLMChatBot {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl Curd for LLMChatTemplate {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
impl Curd for LLMChatSession {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
impl Curd for LLMChatRecord {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
