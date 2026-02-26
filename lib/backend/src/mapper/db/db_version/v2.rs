use chin_sql::{
    SqlBuilder, SqlDeleter, Wheres,
    str_type::{Text, Varchar},
    time_type::TID,
};
use chin_tools::EResult;

use crate::{
    krate::{
        kfile::{InlineKFile, KFileMeta},
        llmchat::LLMChatRecord,
    },
    mapper::db::{KDbExecutor, KDbExecutorBehaiver, KDbRowBehavier, KDbTx},
    model::OtidTableSupport,
};

// move excalidraw files to graph
impl KDbTx<'_> {
    async fn sync_llm_chat_record(&self, history: bool) -> EResult {
        struct ParseType {
            rec: LLMChatRecord,
            thinking: Option<Text>,
        }

        let sb = SqlBuilder::read_all(if !history {
            "llm_chat_record_bak"
        } else {
            "llm_chat_record_hist_bak"
        });
        let c: Vec<ParseType> = self
            .qry_list(sb, |row| {
                Ok(ParseType {
                    rec: LLMChatRecord::try_from(&row)?,
                    thinking: row.try_get("reasoning_content")?,
                })
            })
            .await?;
        for ele in c {
            let thinking = ele
                .thinking
                .map(|e| e.to_string())
                .unwrap_or("".to_string());
            self.exec(
                LLMChatRecord {
                    content:
                        serde_json::json! ({"body": ele.rec.content.as_str(), "thinking":thinking })
                            .to_string()
                            .into(),
                    ..ele.rec
                }
                .to_sql_inserter()
                .table_name(LLMChatRecord::table_name(history)),
            )
            .await?;
        }

        Ok(())
    }

    async fn sync_excalidraw(&self, history: bool) -> EResult {
        struct ParseType {
            otid: TID,
            sid: Varchar<100>,
            content: Option<Text>,
        }

        let sb = SqlBuilder::new()
            .seg("select kfm.otid, kfm.sid, ikf.content from")
            .seg(KFileMeta::table_name(history))
            .seg("kfm left join")
            .seg(InlineKFile::TABLE)
            .seg("ikf on")
            .seg("kfm.")
            .seg(KFileMeta::SID)
            .seg("= ikf.")
            .seg(InlineKFile::SID)
            .seg("where kfm.")
            .seg(KFileMeta::CONTENT_TYPE)
            .seg("in ('chnots/excalidraw-v1', 'excalidraw-v1')");

        let result_list = self
            .qry_list(sb, |row| {
                Ok(ParseType {
                    otid: row.try_get("otid")?,
                    sid: row.try_get("sid")?,
                    content: row.try_get("content")?,
                })
            })
            .await?;

        for pt in &result_list {
            let Some(c) = &pt.content else {
                continue;
            };
            let v: serde_json::Value = serde_json::from_str(c.as_str())?;
            let c =
                <crate::krate::graph::ExcalidrawDataV2Dto as serde::Deserialize>::deserialize(&v)?;
            self.excalidraw_commit(c.try_into()?, pt.otid).await?;
        }

        self.exec(
            SqlDeleter::new(KFileMeta::table_name(history)).r#where(Wheres::r#in(
                KFileMeta::OTID,
                result_list.iter().map(|e| e.otid).collect(),
            )),
        )
        .await?;

        self.exec(SqlDeleter::new(InlineKFile::TABLE).r#where(Wheres::r#in(
            InlineKFile::SID,
            result_list.iter().map(|e| e.sid.clone()).collect(),
        )))
        .await?;

        Ok(())
    }

    pub async fn v2_posthook(&self) -> EResult {
        self.sync_llm_chat_record(true).await?;
        self.sync_llm_chat_record(false).await?;
        self.sync_excalidraw(false).await?;
        self.sync_excalidraw(true).await?;

        Ok(())
    }
}
