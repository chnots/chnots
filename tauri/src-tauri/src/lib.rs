use chnots_core::config::{AttachmentConfig, Config, MapperConfig, ServerConfig, SqliteConfig};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let dir = app
                .path()
                .app_data_dir()
                .expect("couldn't resolve app data dir");
            std::fs::create_dir_all(&dir).unwrap_or_else(|_| panic!("unable to create dir {dir:?}"));

            let config = Config {
                server: Some(ServerConfig { port: 3013 }),
                mapper: MapperConfig::Sqlite(SqliteConfig {
                    filepath: dir.join("chnots.db"),
                    pool_size: 2.into(),
                }),
                file_backup: None,
                attachment: AttachmentConfig {
                    base_dir: dir.join("attach"),
                },
            };

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            tauri::async_runtime::spawn(async move {
                chnots_core::run(config).await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
