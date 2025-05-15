use chin_sql::{IntoSqlSeg, SqlSeg, SqlValueOwned};
use chin_tools::AResult;
use deadpool_sqlite::rusqlite::{self, Connection, Rows};

use crate::{
    flatten_result2, flatten_result3,
    mapper::db::{
        kdb::{KDbBehaiver, KDbConn, KDbConnBehaiver}, KDbConnBehaiverSync, KDbRow
    }, util::result_util::ROSwap,
};

use super::Sqlite;

impl KDbBehaiver for Sqlite {
    async fn conn(&self) -> chin_tools::AResult<crate::mapper::db::kdb::KDbConn> {
        Ok(KDbConn::Sqlite(self.clone()))
    }
}

macro_rules! to_sqlite_params {
    ($params:expr) => {
        $params
            .iter()
            .map(|e| e as &dyn rusqlite::types::ToSql)
            .collect::<Vec<_>>()
            .as_slice()
    };
}

macro_rules! execute_sqlite_query {
    ($self:expr, $seg:expr, $values:expr, $mapper:expr) => {{
        let params: Vec<SqlValueOwned> = $values.into_iter().map(SqlValueOwned::from).collect();

        match $self.prepare(&$seg) {
            Ok(mut stmt) => {
                let rows = stmt.query(to_sqlite_params!(params));

                match rows {
                    Ok(rows) => $mapper(rows),
                    Err(err) => anyhow::bail!(err.to_string()),
                }
            }
            Err(err) => anyhow::bail!(err.to_string()),
        }
    }};
}

impl KDbConnBehaiverSync for Connection {
    fn exec<'a, T: chin_sql::IntoSqlSeg<'a>>(&self, ssb: T) -> chin_tools::AResult<usize> {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("exec {:?}", seg);
        self.execute(&seg, to_sqlite_params!(values))
            .map_err(|err| anyhow::anyhow!(err.to_string()))
    }

    fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(&self, ssb: T, check_count: C) -> AResult<usize>
    where
        C: (FnOnce(usize) -> bool) + Send + 'static,
    {
        let result = self.exec(ssb);
        match result {
            Ok(count) => {
                if check_count(count) {
                    Ok(count)
                } else {
                    Err(anyhow::anyhow!("Count size is not matched {}", count))
                }
            }
            err => Ok(err?),
        }
    }

    fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: FnOnce(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry opt {:?}", seg);
        execute_sqlite_query!(self, seg, values, |mut rows: Rows<'_>| {
            match rows.next() {
                Ok(Some(ok)) => match mapper(KDbRow::Sqlite(ok)) {
                    Ok(t) => Ok(Some(t)),
                    Err(err) => Err(chin_tools::aanyhow!(err)),
                },
                Ok(None) => Ok(None),
                Err(err) => Err(anyhow::anyhow!(err.to_string())),
            }
        })
    }

    fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<E>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: FnOnce(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry one {:?}", seg);
        execute_sqlite_query!(self, seg, values, |mut rows: Rows<'_>| {
            if let Ok(Some(row)) = rows.next() {
                return Ok(mapper(KDbRow::Sqlite(row))?);
            } else {
                anyhow::bail!("Db result is empty");
            }
        })
    }

    fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: Fn(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry list {:?}", seg);
        execute_sqlite_query!(self, seg, values, |mut rows: Rows<'_>| {
            let mut vec = vec![];
            while let Ok(Some(row)) = rows.next() {
                vec.push(mapper(KDbRow::Sqlite(row))?);
            }
            return Ok(vec);
        })
    }
}

impl KDbConnBehaiver for Sqlite {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("exec {:?}", seg);
        let values: Vec<SqlValueOwned> =
            values.into_iter().map(|e| SqlValueOwned::from(e)).collect();

        let result = self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                conn.execute(
                    &seg,
                    values
                        .iter()
                        .map(|e| e as &dyn rusqlite::types::ToSql)
                        .collect::<Vec<&dyn rusqlite::types::ToSql>>()
                        .as_slice(),
                )
            })
            .await
            .map_err(|e| e.to_string());

        flatten_result2!(result)
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

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry one {:?}", seg);
        let values: Vec<SqlValueOwned> =
            values.into_iter().map(|e| SqlValueOwned::from(e)).collect();

        let result = self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                execute_sqlite_query!(conn, seg, values, |mut rows: Rows<'_>| {
                    if let Ok(Some(row)) = rows.next() {
                        return Ok(mapper(KDbRow::Sqlite(row)));
                    } else {
                        anyhow::bail!("Db result is empty");
                    }
                })
            })
            .await
            .map_err(|e| e.to_string());

        flatten_result3!(result)
    }

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry opt {:?}", seg);
        let values: Vec<SqlValueOwned> =
            values.into_iter().map(|e| SqlValueOwned::from(e)).collect();

        let result = self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                execute_sqlite_query!(conn, seg, values, |mut rows: Rows<'_>| {
                    match rows.next() {
                        Ok(Some(ok)) => Ok(Some(mapper(KDbRow::Sqlite(ok)))),
                        Ok(None) => Ok(None),
                        Err(err) => Err(anyhow::anyhow!(err.to_string())),
                    }
                })
            })
            .await
            .map_err(|e| e.to_string());

        match flatten_result2!(result) {
            Ok(v) => {
                v.swap()
            },
            Err(err) => {
                Err(err)
            },
        }
    }

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: chin_sql::IntoSqlSeg<'a>,
        F: (Fn(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        let SqlSeg { seg, values } = ssb.into_sql_seg(chin_sql::DbType::Sqlite)?;
        tracing::info!("qry list {:?}", seg);
        let values: Vec<SqlValueOwned> =
            values.into_iter().map(|e| SqlValueOwned::from(e)).collect();

        let result = self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                execute_sqlite_query!(conn, seg, values, |mut rows: Rows<'_>| {
                    let mut vec = vec![];
                    while let Ok(Some(row)) = rows.next() {
                        vec.push(mapper(KDbRow::Sqlite(row))?);
                    }
                    return Ok(vec);
                })
            })
            .await
            .map_err(|e| e.to_string());

        flatten_result2!(result)
    }
}
