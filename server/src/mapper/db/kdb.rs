use actor_sqlite::client::{ActorSqliteConnClient, ActorSqliteTxClient};
use chin_sql::{
    time_type::TID, DbType, IntoSqlSeg, SqlBuilder, SqlUpdater, SqlValue,
    SqlValueRow, Wheres,
};
use chin_tools::{AResult, EResult};
use deadpool_postgres::{Client, GenericClient, Transaction};
use postgres_types::FromSql;
use tokio_postgres::Row;

use crate::model::omit_tid::OmitTID;

use super::{postgres, sqlite};

pub(crate) trait KDbBehaiver {
    async fn conn(&self) -> AResult<KDbConn>;
}

pub(crate) trait KDbExecutorBehaiver: Send + Sync {
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
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static;

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static;

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static;
}

pub(crate) trait KDbConnBehaiver<'a, Tx>: KDbExecutorBehaiver {
    async fn tx(&'a mut self) -> AResult<Tx>;
}

pub(crate) trait KDbTransactionBehaiver: KDbExecutorBehaiver {
    async fn cmt(self) -> EResult;

    async fn rbk(self) -> EResult;
}

pub(crate) trait KDbRowBehavier<'a, T> {
    fn try_get(&'a self, key: &str) -> AResult<T>;
}

pub(crate) enum KDbRow {
    Postgres(tokio_postgres::Row),
    SqlValue(SqlValueRow),
}

impl<'a, T> KDbRowBehavier<'a, T> for Row
where
    T: FromSql<'a>,
{
    fn try_get(&'a self, key: &str) -> AResult<T> {
        Ok(self.try_get(key)?)
    }
}

impl<'a, E, T> KDbRowBehavier<'a, T> for SqlValueRow
where
    T: TryFrom<SqlValue<'a>, Error = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn try_get(&'a self, key: &str) -> AResult<T> {
        let sv = self.row.get(key);
        if let Some(sv) = sv {
            Ok(sv.clone().try_into()?)
        } else {
            anyhow::bail!("unable to read for key {}", key)
        }
    }
}

impl<'a, T, E> KDbRowBehavier<'a, T> for KDbRow
where
    T: TryFrom<SqlValue<'a>, Error = E> + FromSql<'a>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn try_get(&'a self, key: &str) -> AResult<T> {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::SqlValue(row) => row.try_get(key),
        }
    }
}

pub(crate) enum KDb {
    Sqlite(sqlite::Sqlite),
    Postgres(postgres::Postgres),
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

#[allow(clippy::large_enum_variant)]
pub(crate) enum KDbConn {
    Sqlite(ActorSqliteConnClient),
    Postgres(Client),
}

impl KDbConn {
    pub async fn transaction<'a>(&'a mut self) -> AResult<KDbTx<'a>> {
        match self {
            KDbConn::Sqlite(c) => Ok(KDbTx::Sqlite(c.transaction().await?)),
            KDbConn::Postgres(c) => Ok(KDbTx::Postgres(c.transaction().await?)),
        }
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

impl KDbExecutorBehaiver for KDbConn {
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
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_opt(ssb, mapper))
    }

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_one(ssb, mapper, only_one))
    }

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdb_conn_branch!(self.qry_list(ssb, mapper))
    }

}

impl<'a> KDbConnBehaiver<'a, KDbTx<'a>> for KDbConn {
    async fn tx(&'a mut self) -> AResult<KDbTx<'a>> {
        match self {
            KDbConn::Sqlite(client) => Ok(KDbTx::Sqlite(client.tx().await?)),
            KDbConn::Postgres(client) => Ok(KDbTx::Postgres(client.tx().await?)),
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum KDbTx<'a> {
    Sqlite(ActorSqliteTxClient),
    Postgres(Transaction<'a>),
}

macro_rules! expand_kdbtx_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            KDbTx::Postgres(db) => db.$method($($arg),*).await,
            KDbTx::Sqlite(db) => db.$method($($arg),*).await,
        }
    };
}

impl KDbTransactionBehaiver for KDbTx<'_> {
    async fn cmt(self) -> EResult {
        match self {
            KDbTx::Sqlite(tx) => tx.commit().await?,
            KDbTx::Postgres(tx) => tx.commit().await?,
        }

        Ok(())
    }

    async fn rbk(self) -> EResult {
        match self {
            KDbTx::Sqlite(tx) => tx.rollback().await?,
            KDbTx::Postgres(tx) => tx.rollback().await?,
        }

        Ok(())
    }
}

impl KDbExecutorBehaiver for KDbTx<'_> {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        expand_kdbtx_branch!(self.exec(ssb))
    }

    async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
        &self,
        ssb: T,
        check_count: C,
    ) -> AResult<usize>
    where
        C: (FnOnce(usize) -> bool) + Send + 'static,
    {
        expand_kdbtx_branch!(self.exec_and_check(ssb, check_count))
    }

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdbtx_branch!(self.qry_opt(ssb, mapper))
    }

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdbtx_branch!(self.qry_one(ssb, mapper, only_one))
    }

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_kdbtx_branch!(self.qry_list(ssb, mapper))
    }

}

pub enum KDbExecutor<'e> {
    Conn(&'e KDbConn),
    Tx(&'e KDbTx<'e>),
}

macro_rules! expand_KDbExecutor_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            KDbExecutor::Conn(db) => db.$method($($arg),*).await,
            KDbExecutor::Tx(db) => db.$method($($arg),*).await,
        }
    };
}

impl KDbConn {
    pub fn as_executor(&self) -> KDbExecutor<'_> {
        KDbExecutor::Conn(self)
    }
}

impl KDbTx<'_> {
    pub fn as_executor(&self) -> KDbExecutor<'_> {
        KDbExecutor::Tx(self)
    }
}

impl<'e> KDbExecutorBehaiver for KDbExecutor<'e> {
    async fn exec<'a, T: IntoSqlSeg<'a>>(&self, ssb: T) -> AResult<usize> {
        expand_KDbExecutor_branch!(self.exec(ssb))
    }

    async fn exec_and_check<'a, T: IntoSqlSeg<'a>, C>(
        &self,
        ssb: T,
        check_count: C,
    ) -> AResult<usize>
    where
        C: (FnOnce(usize) -> bool) + Send + 'static,
    {
        expand_KDbExecutor_branch!(self.exec_and_check(ssb, check_count))
    }

    async fn qry_opt<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Option<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_KDbExecutor_branch!(self.qry_opt(ssb, mapper))
    }

    async fn qry_one<'a, E, T, F>(&self, ssb: T, mapper: F, only_one: bool) -> AResult<E>
    where
        T: IntoSqlSeg<'a>,
        F: (FnOnce(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_KDbExecutor_branch!(self.qry_one(ssb, mapper, only_one))
    }

    async fn qry_list<'a, E, T, F>(&self, ssb: T, mapper: F) -> AResult<Vec<E>>
    where
        T: IntoSqlSeg<'a>,
        F: (Fn(KDbRow) -> AResult<E>) + Send + 'static,
        E: Send + 'static,
    {
        expand_KDbExecutor_branch!(self.qry_list(ssb, mapper))
    }

}

pub fn omit_table_tid<'a>(table_name: &'static str, tid: TID) -> SqlUpdater<'a> {
    SqlUpdater::new(table_name)
        .set("omit_tid", OmitTID::now())
        .r#where(Wheres::and([
            Wheres::equal("tid", tid),
            Wheres::equal("omit_tid", OmitTID::never()),
        ]))
}
