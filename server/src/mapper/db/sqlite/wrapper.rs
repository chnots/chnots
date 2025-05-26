use chin_sql::{IntoSqlSeg, SqlSeg, SqlValueOwned};
use chin_tools::AResult;

use crate::mapper::db::{
    kdb::{KDbBehaiver, KDbConn, KDbConnBehaiver},
    KDbRow, KDbTransactionBehaiver,
};

use super::Sqlite;
use actor_sqlite::client::{ActorSqliteConnClient, ActorSqliteTxClient};

impl KDbBehaiver for Sqlite {
    async fn conn(&self) -> chin_tools::AResult<crate::mapper::db::kdb::KDbConn> {
        Ok(KDbConn::Sqlite(self.pool.get().await?))
    }
}

macro_rules! impl_KDbConnBehaiver {
    ($tt:ty) => {
        impl KDbConnBehaiver for $tt {
            async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
                tracing::info!("exec {:?}", seg);
                let values: Vec<SqlValueOwned> =
                    values.into_iter().map(SqlValueOwned::from).collect();

                let count = self.execute(seg, values).await?;
                Ok(count)
            }

            async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
                &self,
                ssb: T,
                check_count: C,
            ) -> AResult<usize>
            where
                C: (FnOnce(usize) -> bool) + Send + 'static,
            {
                let count = self.exec(ssb).await?;
                if check_count(count) {
                    Ok(count)
                } else {
                    Err(anyhow::anyhow!("Count size is not matched {}", count))
                }
            }

            async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
            where
                T: chin_sql::IntoSqlSeg<'a>,
                F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
                E: Send + 'static,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
                tracing::info!("qry one {:?}", seg);
                let values: Vec<SqlValueOwned> =
                    values.into_iter().map(SqlValueOwned::from).collect();

                let rows = self.query(seg, values).await?;

                let length = rows.len();
                if let Some(row) = rows.into_iter().nth(0) {
                    let first_res = mapper(KDbRow::SqlValueRow(row))?;
                    if !only_one {
                        Ok(first_res)
                    } else if length > 1 {
                        anyhow::bail!("Db result more thane one");
                    } else {
                        Ok(first_res)
                    }
                } else {
                    anyhow::bail!("Db result is empty");
                }
            }

            async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
            where
                T: chin_sql::IntoSqlSeg<'a>,
                F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
                E: Send + 'static,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
                tracing::info!("qry one {:?}", seg);
                let values: Vec<SqlValueOwned> =
                    values.into_iter().map(SqlValueOwned::from).collect();

                let rows = self.query(seg, values).await?;

                if let Some(row) = rows.into_iter().nth(0) {
                    let first_res = mapper(KDbRow::SqlValueRow(row))?;
                    Ok(Some(first_res))
                } else {
                    Ok(None)
                }
            }

            async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
            where
                T: chin_sql::IntoSqlSeg<'a>,
                F: (Fn(KDbRow) -> AResult<E>) + Send + 'static,
                E: Send + 'static,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
                tracing::info!("qry one {:?}", seg);
                let values: Vec<SqlValueOwned> =
                    values.into_iter().map(SqlValueOwned::from).collect();

                self.query(seg, values)
                    .await?
                    .into_iter()
                    .map(|r| mapper(KDbRow::SqlValueRow(r)))
                    .collect()
            }
        }
    };
}

impl_KDbConnBehaiver!(ActorSqliteConnClient);
impl_KDbConnBehaiver!(ActorSqliteTxClient);

impl KDbTransactionBehaiver for ActorSqliteTxClient {
    async fn cmt(self) -> chin_tools::EResult {
        self.commit().await?;
        Ok(())
    }

    async fn rbk(self) -> chin_tools::EResult {
        self.rollback().await?;
        Ok(())
    }
}
