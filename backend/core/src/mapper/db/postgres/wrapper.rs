use chin_sql::{IntoSqlSeg, SqlSeg};
use chin_tools::AResult;
use deadpool_postgres::{Client, Transaction};

use crate::{
    mapper::db::{
        kdb::{KDbBehaiver, KDbExecutorBehaiver, KDbRow},
        KDbConnBehaiver, KDbTransactionBehaiver,
    },
    util::result_util::ROSwap,
};

use super::Postgres;
use crate::to_pgsql_params;

macro_rules! impl_KDbExecutorBehaiver {
    ($ty:ident $(<$($lt:lifetime),+>)?) => {
        impl $(<$($lt),+>)? KDbExecutorBehaiver for $ty $(<$($lt),+>)? {
            async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
                tracing::info!("exec_and_check {:?} {:?}", seg, values);
                let count = self.execute(&seg, to_pgsql_params!(values)).await?;

                Ok(count as usize)
            }

            async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
                &self,
                ssb: T,
                check_count: C,
            ) -> AResult<usize>
            where
                C: (FnOnce(usize) -> bool) + Send + 'static,
            {
                match self.exec(ssb).await {
                    Ok(count) => {
                        if check_count(count) {
                            Ok(count)
                        } else {
                            Err(anyhow::anyhow!("Count size is not matched {}", count))
                        }
                    }
                    err => err,
                }
            }

            async fn qry_opt<'a, E, T, F>(
                &self,
                ssb: T,
                mapper: F,
            ) -> chin_tools::AResult<Option<E>>
            where
                T: IntoSqlSeg<'a>,
                F: FnOnce(KDbRow) -> AResult<E>,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
                tracing::info!("query opt {:?}", seg);
                let result = self.query_opt(&seg, to_pgsql_params!(values)).await?;
                result.map(|e| mapper(KDbRow::Postgres(e))).swap()
            }

            async fn qry_one<'a, E, T, F>(
                &self,
                ssb: T,
                mapper: F,
                only_one: bool,
            ) -> chin_tools::AResult<E>
            where
                T: IntoSqlSeg<'a>,
                F: FnOnce(KDbRow) -> AResult<E>,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
                tracing::info!("query one {:?}", seg);
                if !only_one {
                    let result = self.query_one(&seg, to_pgsql_params!(values)).await?;
                    Ok(mapper(KDbRow::Postgres(result))?)
                } else {
                    let mut result = self.query(&seg, to_pgsql_params!(values)).await?;
                    let len = result.len();
                    if len > 1 {
                        Err(anyhow::anyhow!("More than one line"))
                    } else if len == 1 {
                        Ok(mapper(KDbRow::Postgres(result.swap_remove(0)))?)
                    } else {
                        Err(anyhow::anyhow!("The result is empty"))
                    }
                }
            }

            async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> chin_tools::AResult<Vec<E>>
            where
                T: IntoSqlSeg<'a>,
                F: Fn(KDbRow) -> AResult<E>,
            {
                let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
                tracing::info!("query list {:?}, values {:?}", seg, values);
                let result = self.query(&seg, to_pgsql_params!(values)).await?;
                result
                    .into_iter()
                    .map(|e| mapper(KDbRow::Postgres(e)))
                    .collect()
            }
        }
    };
}

impl KDbBehaiver for Postgres {
    async fn conn(&self) -> chin_tools::AResult<crate::mapper::db::kdb::KDbConn> {
        Ok(crate::mapper::db::KDbConn::Postgres(self.client().await?))
    }
}

impl_KDbExecutorBehaiver! {Client}
impl_KDbExecutorBehaiver! {Transaction<'b>}

impl<'b> KDbConnBehaiver<'b, Transaction<'b>> for Client {
    async fn tx(&'b mut self) -> AResult<Transaction<'b>> {
        Ok(self.transaction().await?)
    }
}

impl<'b> KDbTransactionBehaiver for Transaction<'b> {
    async fn cmt(self) -> chin_tools::EResult {
        self.commit().await?;
        Ok(())
    }

    async fn rbk(self) -> chin_tools::EResult {
        self.rollback().await?;
        Ok(())
    }
}
