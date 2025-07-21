use chin_sql::{ChinSqlError, CreateTableSql, CreateTableSqlOwned, SqlBuilder, SqlDeleter, Wheres};
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver};

pub(crate) async fn create_tables(cts: Vec<&CreateTableSql>, kdb: &KDb) -> EResult {
    let sqls: Result<Vec<Vec<String>>, ChinSqlError> = cts
        .iter()
        .map(|cts| cts.to_owned_sql().sqls(kdb.get_db_type()))
        .collect();
    let sqls = sqls?.into_iter().flat_map(|c| c.into_iter());

    for sql in sqls {
        kdb.conn().await?.exec(sql).await?;
    }

    Ok(())
}

pub(crate) async fn to_ommitted_table(
    mut create_table: CreateTableSqlOwned,
) -> CreateTableSqlOwned {
    let pkey = create_table.pkey.clone();
    create_table.pkey.clear();

    for ele in pkey {
        create_table.keys.push((ele.clone(), vec![ele]));
    }

    create_table.table_name = create_table.table_name.omitted_table_name();

    create_table
}

pub(crate) trait OmittedTableName {
    fn omitted_table_name(&self) -> String;
}

impl OmittedTableName for &'_ str {
    fn omitted_table_name(&self) -> String {
        format!("{self}_formatted")
    }
}

impl OmittedTableName for String {
    fn omitted_table_name(&self) -> String {
        format!("{self}_formatted")
    }
}

impl KDbExecutor<'_> {
    pub(crate) async fn omit_rows(&self, table_name: &str, condition: Wheres<'_>) -> EResult {
        self.copy_into_omit_table(table_name, condition.clone()).await?;
        let delete_sql = SqlDeleter::new(table_name).r#where(condition);
        self.exec(delete_sql).await?;
        Ok(())
    }

    pub(crate) async fn copy_into_omit_table(
        &self,
        table_name: &str,
        condition: Wheres<'_>,
    ) -> EResult {
        let insert_sql = SqlBuilder::new()
            .sov(format!(
                "insert into {} select * from {} where",
                table_name.omitted_table_name(),
                table_name
            ))
            .r#where(condition.clone());
        self.exec(insert_sql).await?;
        Ok(())
    }
}
