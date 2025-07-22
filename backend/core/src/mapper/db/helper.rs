use chin_sql::{ChinSqlError, CreateTableSqlOwned, SqlBuilder, SqlDeleter, Wheres};
use chin_tools::{AResult, EResult};
use itertools::Itertools;
use log::info;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver};

pub(crate) async fn create_tables(cts: Vec<CreateTableSqlOwned>, kdb: &KDb) -> EResult {
    for c in cts.iter() {
        if c.table_name.contains("_hist") {
            log::info!(
                "example sql: insert into {}({}) select {} from {}_bak where omit_tid <> -404;",
                c.table_name,
                c.fields.iter().map(|f| f.name).join(", "),
                c.fields.iter().map(|f| f.name).join(", "),
                c.table_name.replace("_hist", "")
            );
        } else {
            log::info!(
                "example sql: insert into {}({}) select {} from {}_bak where omit_tid = -404;",
                c.table_name,
                c.fields.iter().map(|f| f.name).join(", "),
                c.fields.iter().map(|f| f.name).join(", "),
                c.table_name.replace("_hist", "")
            );
        }
    }
    let sqls: Result<Vec<Vec<String>>, ChinSqlError> = cts
        .into_iter()
        .map(|cts| cts.sqls(kdb.get_db_type()))
        .collect();
    let sqls = sqls?.into_iter().flat_map(|c| c.into_iter());

    for sql in sqls {
        kdb.conn().await?.exec(sql).await?;
    }

    Ok(())
}

pub(crate) fn to_ommitted_table(mut create_table: CreateTableSqlOwned) -> CreateTableSqlOwned {
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
        format!("{self}_hist")
    }
}

impl OmittedTableName for String {
    fn omitted_table_name(&self) -> String {
        format!("{self}_hist")
    }
}

impl KDbExecutor<'_> {
    pub(crate) async fn omit_rows(
        &self,
        table_name: &str,
        fields: &[&str],
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        let count = self
            .copy_into_omit_table(table_name, fields, condition.clone())
            .await?;
        if count > 0 {
            let delete_sql = SqlDeleter::new(table_name).r#where(condition);
            self.exec(delete_sql).await
        } else {
            Ok(0)
        }
    }

    pub(crate) async fn copy_into_omit_table(
        &self,
        table_name: &str,
        fields: &[&str],
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        let insert_sql = SqlBuilder::new()
            .sov(format!(
                "insert into {}({}) select {} from {}",
                table_name.omitted_table_name(),
                fields.join(","),
                fields.join(","),
                table_name
            ))
            .r#where(condition.clone());
        let count = self.exec(insert_sql).await?;
        Ok(count)
    }
}
