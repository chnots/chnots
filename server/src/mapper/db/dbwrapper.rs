use chin_sql::{DateFixed, DbType, IntoSqlSeg, SqlSegBuilder};
use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset};
use deadpool_postgres::Client;
use deadpool_sqlite::rusqlite;

use crate::mapper::DeserializeMapper;

use super::{postgres, sqlite};

pub trait KDbConnBehaiver {
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

pub enum KDbConn {
    Sqlite(sqlite::Sqlite),
    Postgres(Client),
}

pub trait KDbBehaiver {
    async fn conn(&self) -> AResult<KDbConn>;
}

pub enum KDb {
    Sqlite(sqlite::Sqlite),
    Postgres(postgres::Postgres),
}

pub trait KDbRowBehavier {
    fn try_get<T>(&self, key: &str) -> AResult<T>;
}

pub enum KDbRow<'a> {
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
    pub fn db_type(&self) -> DbType {
        match self {
            KDb::Sqlite(_) => DbType::Sqlite,
            KDb::Postgres(_) => DbType::Postgres,
        }
    }

    pub async fn create_table(&self, sql: &str) -> EResult {
        self.conn()
            .await?
            .exec(SqlSegBuilder::new().raw(sql))
            .await?;
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

macro_rules! expand_kdb_row_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            KDbRow::Postgres(db) => db.$method($($arg),*),
            KDbRow::Sqlite(db) => db.$method($($arg),*),
        }
    };
}

impl<'a, 'b> KDbRow<'a> {
    pub fn try_get<T>(&'b self, key: &str) -> AResult<T>
    where
        T: postgres_types::FromSql<'b> + rusqlite::types::FromSql,
    {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::Sqlite(row) => Ok(row.get(key)?),
        }
    }

    pub fn try_get_df(&'b self, key: &str) -> AResult<DateTime<FixedOffset>> {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::Sqlite(row) => Ok(row.get::<&str, DateFixed>(key)?.fixed_offset()),
        }
    }

    pub fn try_get_df_opt(&'b self, key: &str) -> AResult<Option<DateTime<FixedOffset>>> {
        match self {
            KDbRow::Postgres(row) => Ok(row.try_get(key)?),
            KDbRow::Sqlite(row) => Ok(row
                .get::<&str, Option<DateFixed>>(key)
                .map(|e| e.map(|df| df.fixed_offset()))?),
        }
    }
}

impl<'a> DeserializeMapper for KDbRow<'a> {
    fn to_chnot_meta(self) -> AResult<crate::model::db::chnot::ChnotMetadata> {
        expand_kdb_row_branch!(self.to_chnot_meta())
    }

    fn to_chnot_record(self) -> AResult<crate::model::db::chnot::ChnotRecord> {
        expand_kdb_row_branch!(self.to_chnot_record())
    }

    fn to_chnot_tag(self) -> AResult<crate::model::db::chnot::ChnotTag> {
        expand_kdb_row_branch!(self.to_chnot_tag())
    }

    fn to_llmchat_bot(self) -> AResult<crate::model::db::llmchat::LLMChatBot> {
        expand_kdb_row_branch!(self.to_llmchat_bot())
    }

    fn to_llmchat_template(self) -> AResult<crate::model::db::llmchat::LLMChatTemplate> {
        expand_kdb_row_branch!(self.to_llmchat_template())
    }

    fn to_llmchat_session(self) -> AResult<crate::model::db::llmchat::LLMChatSession> {
        expand_kdb_row_branch!(self.to_llmchat_session())
    }

    fn to_llmchat_record(self) -> AResult<crate::model::db::llmchat::LLMChatRecord> {
        expand_kdb_row_branch!(self.to_llmchat_record())
    }

    fn to_namespace_record(self) -> AResult<crate::model::db::namespace::NamespaceRecord> {
        expand_kdb_row_branch!(self.to_namespace_record())
    }

    fn to_namespace_relation(self) -> AResult<crate::model::db::namespace::NamespaceRelation> {
        expand_kdb_row_branch!(self.to_namespace_relation())
    }

    fn to_resource(self) -> AResult<crate::model::db::resource::Resource> {
        expand_kdb_row_branch!(self.to_resource())
    }

    fn to_kv(self) -> AResult<crate::model::db::kv::KV> {
        expand_kdb_row_branch!(self.to_kv())
    }

    fn to_inline_resource(self) -> AResult<crate::model::db::resource::InlineResource> {
        expand_kdb_row_branch!(self.to_inline_resource())
    }
}
