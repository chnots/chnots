use chin_sql::{ChinSqlError, CreateTableSqlOwned, SqlBuilder, SqlDeleter, Wheres};
use chin_tools::{AResult, EResult};

use crate::{
    mapper::db::{KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver},
    model::KOtidSupport,
};

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

impl KDbExecutor<'_> {
    pub(crate) async fn omit_rows<T: KOtidSupport>(&self, condition: Wheres<'_>) -> AResult<usize> {
        let count = self.copy_into_omit_table::<T>(condition.clone()).await?;
        if count > 0 {
            let delete_sql = SqlDeleter::new(T::table_name(false)).r#where(condition);
            self.exec(delete_sql).await
        } else {
            Ok(0)
        }
    }

    pub(crate) async fn copy_into_omit_table<T: KOtidSupport>(
        &self,
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        let fields_comma = T::all_columns().join(",");
        let insert_sql = SqlBuilder::new()
            .seg(format!(
                "insert into {}({}) select {} from {}",
                T::table_name(true),
                &fields_comma,
                &fields_comma,
                T::table_name(false)
            ))
            .r#where(condition.clone());

        let count = self.exec(insert_sql).await?;
        Ok(count)
    }
}
