pub mod v2;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;

use chin_sql::{DbType, GenerateTableSchema, OrderBy, SqlBuilder, Wheres, time_type::TID};
use log::info;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use crate::{
    magics::DB_VERSION,
    mapper::{
        Curd,
        db::{
            KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTx,
            helper::{Ddls, create_tables},
            kdb::KDbTransactionBehaiver,
        },
    },
};

use chin_tools::{AResult, EResult};

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct DbVersionTransient {
    #[gts_primary]
    pub version: i64,
    #[gts_type = "i64"]
    pub tid: TID,
}

impl Curd for DbVersionTransient {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.version)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl TryFrom<&KDbRow> for DbVersionTransient {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            version: value.try_get(Self::VERSION)?,
            tid: value.try_get(Self::TID)?,
        })
    }
}

impl KDb {
    pub async fn ensure_table_db_version(&self) -> EResult {
        let mut create_sql = DbVersionTransient::create_sql().to_owned_sql();
        create_sql.table_name = DbVersionTransient::TABLE.to_string();
        create_tables(Ddls::new().with_ddl(create_sql), self).await
    }

    pub async fn sync_db_version(&self) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.migrate_db(DB_VERSION.trim().parse::<i64>()?, self.get_db_type())
            .await?;
        tx.cmt().await?;
        Ok(())
    }
}

impl KDbTx<'_> {
    async fn get_current_version(&self) -> AResult<Option<i64>> {
        let result = self
            .qry_opt(
                SqlBuilder::read_all(DbVersionTransient::TABLE)
                    .order_by([OrderBy::Desc(DbVersionTransient::VERSION.into())])
                    .limit(1),
                |row| -> AResult<i64> {
                    let version: i64 = row.try_get(DbVersionTransient::VERSION)?;
                    Ok(version)
                },
            )
            .await?;
        Ok(result)
    }

    async fn set_version(&self, version: i64) -> EResult {
        self.exec(
            DbVersionTransient {
                version,
                tid: TID::default(),
            }
            .to_sql_inserter(),
        )
        .await?;
        Ok(())
    }

    async fn migrate_db(&self, program_version: i64, db_type: DbType) -> EResult {
        let db_version = self.get_current_version().await?.unwrap_or(0);
        log::info!("migrate db {} -- {}", program_version, db_version);

        if db_version > program_version {
            anyhow::bail!(
                "Database version ({}) is higher than program version ({}). Please upgrade the application.",
                db_version,
                program_version
            );
        }

        if db_version == program_version {
            info!("Database is up to date. Version: {}", db_version);
            return Ok(());
        }

        let sql_files = Self::read_migration_sqls(db_type)?;
        let mut max_version = db_version;

        for (version, sql) in sql_files {
            log::info!("exec ver: {}", version);

            if version > db_version && version <= program_version {
                info!("Migrating database to version {}", version);
                for stmt in sql.split(";\n") {
                    let stmt = stmt.trim();
                    if !stmt.is_empty() {
                        self.exec(stmt).await?;
                    }
                }
                if version == 2 {
                    self.v2_posthook().await?;
                }
                self.set_version(version).await?;
                max_version = max_version.max(version);
                info!("Migrated to version {}", version);
            }
        }

        if max_version > db_version {
            info!(
                "Database migration completed. Current version: {}",
                max_version
            );
        }

        if max_version != program_version {
            anyhow::bail!(
                "Failed to migrate database to program version ({}). Current version: {}",
                program_version,
                max_version
            );
        }

        Ok(())
    }

    fn read_migration_sqls(db_type: DbType) -> AResult<BTreeMap<i64, String>> {
        let mut sqls = BTreeMap::new();

        for file in MigrationSqls::iter() {
            let file_path = Path::new(file.as_ref());
            if file_path.extension().and_then(|e| e.to_str()) == Some("sql") {
                let filename = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Invalid filename: {:?}", file))?;
                let mut suffix = match db_type {
                    DbType::Sqlite => "-sqlite",
                    DbType::Postgres => "-postgres",
                };

                if filename.ends_with("-all") {
                    suffix = "-all";
                } else if !filename.ends_with(suffix) {
                    continue;
                }

                if let Some(version_str) = filename
                    .strip_prefix('v')
                    .and_then(|version| version.strip_suffix(suffix))
                    && let Ok(version) = version_str.parse::<i64>()
                {
                    let embedded_file = MigrationSqls::get(&file).ok_or_else(|| {
                        anyhow::anyhow!("Unable to find embedded file: {:?}", file)
                    })?;

                    let sql = match embedded_file.data {
                        Cow::Borrowed(bytes) => String::from_utf8_lossy(bytes).to_string(),
                        Cow::Owned(bytes) => String::from_utf8(bytes)?,
                    };

                    sqls.insert(version, sql);
                }
            }
        }

        Ok(sqls)
    }
}

#[derive(RustEmbed)]
#[folder = "../../data/sqls"]
struct MigrationSqls;

#[test]
fn print_ddls() {
    for db_type in [DbType::Postgres, DbType::Sqlite] {
        use crate::mapper::db::HistCreateSql;
        let ddls = Ddls::new()
            .with_ddls(crate::krate::kspace::KSpace::ddls())
            .with_ddls(crate::krate::graph::GraphMeta::ddls())
            .with_ddls(crate::krate::kkv::KKV::ddls())
            .with_ddls(crate::krate::chnot::ChnotThreadOrder::ddls())
            .with_ddls(crate::krate::chnot::ChnotMeta::ddls())
            .with_ddls(crate::krate::kfile::KFileMeta::ddls())
            .with_ddls(crate::krate::llmchat::LLMChatBot::ddls())
            .with_ddls(crate::krate::llmchat::LLMChatTemplate::ddls())
            .with_ddls(crate::krate::llmchat::LLMChatSession::ddls())
            .with_ddls(crate::krate::llmchat::LLMChatRecord::ddls())
            .with_ddls(crate::krate::mdwt::MdwtRecord::ddls())
            .with_ddls(crate::krate::mdwt::MdwtTag::ddls())
            .with_ddls(crate::krate::toent::po::ToentTodo::ddls())
            .with_ddls(crate::krate::toent::po::ToentEvent::ddls())
            .with_ddls(crate::krate::toent::po::ToentInst::ddls())
            .with_ddls(crate::krate::ktab::KTabMeta::ddls())
            .with_ddls(crate::krate::ktab::KTabCellText::ddls())
            .with_ddls(crate::krate::ktab::KTabCellDecimal::ddls())
            .with_ddls(crate::krate::ktab::KTabCellDate::ddls());
        let sqls: Result<Vec<Vec<String>>, chin_sql::ChinSqlError> =
            ddls.ddls.into_iter().map(|cts| cts.sqls(db_type)).collect();
        let sqls: Vec<String> = sqls
            .unwrap()
            .into_iter()
            .flat_map(|c| c.into_iter())
            .collect();
        println!("\n\n{}", sqls.join(";\n"));
    }
}
