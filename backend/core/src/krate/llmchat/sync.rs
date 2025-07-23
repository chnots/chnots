use std::path::PathBuf;

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState, dump_table_to_file, impl_sync_operator, krate::{
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        sync::filedumper::StartType,
    }
};

impl ShareAppState {
    pub async fn dump_llmchat_to_file(&self, start_type: StartType) -> EResult {
        let mapper = &self.mapper;
        let Some(backup_dir) = self
            .config
            .file_backup
            .as_ref()
            .map(|c| c.backup_dir.clone())
        else {
            return Ok(());
        };

        let backup_dir: PathBuf = backup_dir.into();
        let end_in = TID::default();
        dump_table_to_file!(mapper, LLMChatBot, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, LLMChatRecord, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, LLMChatTemplate, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, LLMChatSession, start_type, backup_dir, end_in);
        Ok(())
    }
}

impl_sync_operator! { LLMChatBot, otid }
impl_sync_operator! { LLMChatRecord, otid }
impl_sync_operator! { LLMChatTemplate, otid }
impl_sync_operator! { LLMChatSession, otid }

