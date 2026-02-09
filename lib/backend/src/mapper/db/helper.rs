use chin_sql::{ChinSqlError, CreateTableSqlOwned};
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver};

pub(crate) async fn create_tables(cts: Ddls, kdb: &KDb) -> EResult {
    let sqls: Result<Vec<Vec<String>>, ChinSqlError> = cts
        .ddls
        .into_iter()
        .map(|cts| cts.sqls(kdb.get_db_type()))
        .collect();
    let sqls = sqls?.into_iter().flat_map(|c| c.into_iter());

    for sql in sqls {
        kdb.conn().await?.exec(sql).await?;
    }

    Ok(())
}

pub(crate) async fn print_ddls(cts: Ddls, kdb: &KDb) -> EResult {
    let sqls: Result<Vec<Vec<String>>, ChinSqlError> = cts
        .ddls
        .into_iter()
        .map(|cts| cts.sqls(kdb.get_db_type()))
        .collect();
    let sqls: Vec<String> = sqls?.into_iter().flat_map(|c| c.into_iter()).collect();
    println!("\n\n{}", sqls.join(";\n"));

    Ok(())
}

pub struct Ddls {
    pub ddls: Vec<CreateTableSqlOwned>,
}

impl Ddls {
    pub fn new() -> Self {
        Self {
            ddls: Default::default(),
        }
    }

    pub(crate) fn with_ddls(mut self, ddls: Vec<CreateTableSqlOwned>) -> Self {
        self.ddls.extend(ddls);
        self
    }

    pub(crate) fn with_ddl(mut self, ddl: CreateTableSqlOwned) -> Self {
        self.ddls.push(ddl);
        self
    }
}
