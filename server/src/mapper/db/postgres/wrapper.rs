use chin_sql::{IntoSqlSeg, SqlSeg};
use chin_tools::AResult;
use deadpool_postgres::Client;
use tokio_postgres::Transaction;

use crate::{
    mapper::db::kdb::{KDbBehaiver, KDbConnBehaiver, KDbRow},
    util::result_util::ROSwap,
};

use super::Postgres;
use crate::to_sql;

impl KDbBehaiver for Postgres {
    async fn conn(&self) -> chin_tools::AResult<crate::mapper::db::kdb::KDbConn> {
        Ok(crate::mapper::db::KDbConn::Postgres(self.client().await?))
    }
}

impl<'b> KDbConnBehaiver for Transaction<'b> {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("exec_and_check {:?} {:?}", seg, values);
        let count = self.execute(&seg, to_sql!(values)).await?;

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

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let ss = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query opt {:?}", ss.seg);
        let result = self.query_opt(&ss.seg, to_sql!(ss.values)).await?;
        result.map(|e| mapper(KDbRow::Postgres(e))).swap()
    }

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query one {:?}", seg);
        if !only_one {
            let result = self.query_one(&seg, to_sql!(values)).await?;
            Ok(mapper(KDbRow::Postgres(result))?)
        } else {
            let mut result = self.query(&seg, to_sql!(values)).await?;
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

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query list {:?}", seg);
        let result = self.query(&seg, to_sql!(values)).await?;
        result
            .into_iter()
            .map(|e| mapper(KDbRow::Postgres(e)))
            .collect()
    }
}

impl KDbConnBehaiver for Client {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("exec_and_check {:?} {:?}", seg, values);
        let count = self.execute(&seg, to_sql!(values)).await?;

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

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> chin_tools::AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: FnOnce(KDbRow<'a>) -> AResult<E>,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query opt {:?}", seg);
        let result = self.query_opt(&seg, to_sql!(values)).await?;
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
        F: FnOnce(KDbRow<'a>) -> AResult<E>,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query one {:?}", seg);
        if !only_one {
            let result = self.query_one(&seg, to_sql!(values)).await?;
            Ok(mapper(KDbRow::Postgres(result))?)
        } else {
            let mut result = self.query(&seg, to_sql!(values)).await?;
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
        F: Fn(KDbRow<'a>) -> AResult<E>,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Postgres)?;
        tracing::info!("query list {:?}, values {:?}", seg, values);
        let result = self.query(&seg, to_sql!(values)).await?;
        result
            .into_iter()
            .map(|e| mapper(KDbRow::Postgres(e)))
            .collect()
    }
}
