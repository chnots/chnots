use chin_sql::{ChinSqlError, CreateTableSql};
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver};

pub(crate) async fn create_tables(cts: Vec<&CreateTableSql>, kdb: &KDb) -> EResult {
    let sqls: Result<Vec<Vec<String>>, ChinSqlError> =
        cts.iter().map(|cts| cts.to_owned_sql().sqls(kdb.get_db_type())).collect();
    let sqls = sqls?.into_iter().flat_map(|c| c.into_iter());

    for sql in sqls {
        kdb.conn().await?.exec(sql).await?;
    }

    Ok(())
}
