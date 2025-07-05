use chnots_core::config::{AttachmentConfig, Config, MapperConfig, ServerConfig, SqliteConfig};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let workdir = "/home/chin/chnots-tauri/";
    let config = Config {
        server: Some(ServerConfig {
            port: 3013,
        }),
        mapper: MapperConfig::Sqlite(SqliteConfig {
            filepath: format!("{workdir}chnots.db"),
            pool_size: 2.into(),
        }),
        file_backup: None,
        attachment: AttachmentConfig {
            base_dir: workdir.to_string(),
        },
    };

    tauri::Builder::default()
        .setup(|app| {
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
