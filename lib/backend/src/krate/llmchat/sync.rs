use std::path::PathBuf;

use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        sync::filedumper::StartType,
    },
};

impl ShareAppState {
    pub async fn dump_llmchat_to_file(
        &self,
        start_type: StartType,
        backup_dir: &PathBuf,
    ) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, LLMChatBot>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, LLMChatRecord>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, LLMChatTemplate>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, LLMChatSession>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_llmchat(&self, endpoint: &crate::krate::sync::po::SyncEndpoint) -> EResult {
        self.sync_one_otid_table_only::<LLMChatBot>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<LLMChatRecord>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<LLMChatSession>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<LLMChatTemplate>(endpoint)
            .await?;

        Ok(())
    }
}
