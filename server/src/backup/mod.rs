use serde::Deserialize;

use crate::app::ShareAppState;

#[derive(Debug, Deserialize, Clone)]
pub struct BackupConfig {
    backup_dir: String,
}

pub struct BackupWorker {
    app_state: ShareAppState,
}
