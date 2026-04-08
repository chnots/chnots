use chin_sql::{SqlBuilder, SqlUpdater, Wheres};
use chin_tools::EResult;

use crate::{
    krate::llmchat::LLMChatRecord,
    mapper::db::{KDbExecutorBehaiver, KDbTx},
    model::OtidTableSupport,
};

impl KDbTx<'_> {
    pub async fn v8_migrate_llmchat_content(&self) -> EResult {
        self.migrate_llmchat_content_format(false).await?;
        self.migrate_llmchat_content_format(true).await?;
        Ok(())
    }

    async fn migrate_llmchat_content_format(&self, history: bool) -> EResult {
        #[derive(serde::Deserialize)]
        struct OldFormat {
            body: String,
            thinking: Option<String>,
        }

        let sb = SqlBuilder::read_all(LLMChatRecord::table_name(history));
        let records: Vec<LLMChatRecord> = self
            .qry_list(sb, |row| LLMChatRecord::try_from(&row))
            .await?;

        for rec in records {
            if let Ok(old) = serde_json::from_str::<OldFormat>(rec.content.as_str()) {
                let mut blocks: Vec<serde_json::Value> = Vec::new();
                if let Some(thinking) = &old.thinking {
                    if !thinking.is_empty() {
                        blocks.push(serde_json::json!({"type": "thinking", "data": thinking}));
                    }
                }
                blocks.push(serde_json::json!({"type": "content", "data": old.body}));

                self.exec(
                    SqlUpdater::new(LLMChatRecord::table_name(history))
                        .set(LLMChatRecord::CONTENT, serde_json::to_string(&blocks)?)
                        .r#where(Wheres::equal(LLMChatRecord::OTID, rec.otid)),
                )
                .await?;
            }
        }

        Ok(())
    }
}
