use chin_sql::{SqlBuilder, SqlUpdater, Wheres};
use chin_tools::EResult;

use crate::{
    krate::llmchat::LLMChatRecord,
    mapper::db::{KDbExecutorBehaiver, KDbTx},
    model::OtidTableSupport,
};

impl KDbTx<'_> {
    pub async fn v9_migrate_llmchat_record_content(&self) -> EResult {
        self.migrate_llmchat_record_to_obj(false).await?;
        self.migrate_llmchat_record_to_obj(true).await?;
        Ok(())
    }

    async fn migrate_llmchat_record_to_obj(&self, history: bool) -> EResult {
        let sb = SqlBuilder::read_all(LLMChatRecord::table_name(history));
        let records: Vec<LLMChatRecord> = self
            .qry_list(sb, |row| LLMChatRecord::try_from(&row))
            .await?;

        for rec in records {
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(rec.content.as_str()) {
                let new_content = serde_json::json!({
                    "usage": {},
                    "parts": arr
                });

                self.exec(
                    SqlUpdater::new(LLMChatRecord::table_name(history))
                        .set(LLMChatRecord::CONTENT, serde_json::to_string(&new_content)?)
                        .r#where(Wheres::equal(LLMChatRecord::OTID, rec.otid)),
                )
                .await?;
            }
        }

        Ok(())
    }
}
