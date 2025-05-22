use chin_sql::{DateFixedOffset, DbType, IntoSqlSeg, SqlReader};
use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset};
use deadpool_postgres::Client;
use deadpool_sqlite::rusqlite;

use super::{postgres, sqlite};

pub(crate) trait KDbConnBehaiver {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize>;

    async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
        &self,
        ssb: T,
        check_count: C,
    ) -> AResult<usize>
    where
        C: (FnOnce(usize) -> bool) + Send + 'static;

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static;

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static;

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static;
}

pub(crate) trait KDbConnBehaiverSync {
    fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize>;

    fn exec_and_check<'a, T, C>(&self, ssb: T, check_count: C) -> AResult<usize>
    where
        T: IntoSqlSeg<'a>,
        C: FnOnce(usize) -> bool + Send + 'static;

    fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: FnOnce(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static;

    fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: FnOnce(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static;

    fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: Fn(KDbRow<'_>) -> AResult<E>,
        E: Send + 'static;
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum KDbConn {
    Sqlite(sqlite::Sqlite),
    Postgres(Client),
}

pub(crate) trait KDbBehaiver {
    async fn conn(&self) -> AResult<KDbConn>;
}

pub(crate) enum KDb {
    Sqlite(sqlite::Sqlite),
    Postgres(postgres::Postgres),
}

pub(crate) trait KDbRowBehavier<'b, T> {
    fn try_get(&'b self, key: &str) -> AResult<T>;
}

pub(crate) enum KDbRow<'a> {
    Postgres(tokio_postgres::Row),
    Sqlite(&'a rusqlite::Row<'a>),
}

macro_rules! expand_kdb_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            KDb::Postgres(db) => db.$method($($arg),*).await,
            KDb::Sqlite(db) => db.$method($($arg),*).await,
        }
    };
}

impl KDbBehaiver for KDb {
    async fn conn(&self) -> AResult<KDbConn> {
        expand_kdb_branch!(self.conn())
    }
}

impl KDb {
    pub(crate) fn db_type(&self) -> DbType {
        match self {
            KDb::Sqlite(_) => DbType::Sqlite,
            KDb::Postgres(_) => DbType::Postgres,
        }
    }

    pub(crate) async fn create_table(&self, sql: &str) -> EResult {
        self.conn().await?.exec(SqlReader::new().raw(sql)).await?;
        Ok(())
    }
}

macro_rules! expand_kdb_conn_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            KDbConn::Postgres(db) => db.$method($($arg),*).await,
            KDbConn::Sqlite(db) => db.$method($($arg),*).await,
        }
    };
}

impl KDbConnBehaiver for KDbConn {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        expand_kdb_conn_branch!(self.exec(ssb))
    }

    async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
        &self,
        ssb: T,
        check_count: C,
    ) -> AResult<usize>
    where
        C: (FnOnce(usize) -> bool) + Send + 'static,
    {
        expand_kdb_conn_branch!(self.exec_and_check(ssb, check_count))
    }

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_opt(ssb, mapper))
    }

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_one(ssb, mapper, only_one))
    }

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow<'_>) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_list(ssb, mapper))
    }
}

macro_rules! common_try_get {
    ($tp:tt) => {
        impl<'a, 'b> KDbRowBehavier<'b, $tp> for KDbRow<'a> {
            fn try_get(&'b self, key: &str) -> AResult<$tp> {
                match self {
                    KDbRow::Postgres(row) => Ok(row.try_get(key)?),
                    KDbRow::Sqlite(row) => Ok(row.get(key)?),
                }
            }
        }

        impl<'a, 'b> KDbRowBehavier<'b, Option<$tp>> for KDbRow<'a> {
            fn try_get(&'b self, key: &str) -> AResult<Option<$tp>> {
                match self {
                    KDbRow::Postgres(row) => Ok(row.try_get(key)?),
                    KDbRow::Sqlite(row) => Ok(row.get(key)?),
                }
            }
        }
    };
}

common_try_get! {i64}
common_try_get! {i32}
common_try_get! {f64}
common_try_get! {String}
common_try_get! {bool}

impl<'a, 'b> KDbRowBehavier<'b, DateTime<FixedOffset>> for KDbRow<'a> {
    fn try_get(&'b self, key: &str) -> AResult<DateTime<FixedOffset>> {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::Sqlite(row) => Ok(row.get::<&str, DateFixedOffset>(key)?.fixed_offset()),
        }
    }
}

impl<'a, 'b> KDbRowBehavier<'b, Option<DateTime<FixedOffset>>> for KDbRow<'a> {
    fn try_get(&'b self, key: &str) -> AResult<Option<DateTime<FixedOffset>>> {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::Sqlite(row) => Ok(row
                .get::<&str, Option<DateFixedOffset>>(key)
                .map(|e| e.map(|df| df.fixed_offset()))?),
        }
    }
}

