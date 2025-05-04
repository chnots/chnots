use chin_sql::{DateFixed, DbType, IntoSqlSeg, SqlReader};
use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset};
use deadpool_postgres::Client;
use deadpool_sqlite::rusqlite;

use super::{postgres, sqlite};
use crate::mapper::DeserializeMapper;
use crate::model::db::{
    chnot::*,
    llmchat::*,
    namespace::*,
    resource::KV,
    resource::{InlineResource, Resource},
};

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
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.try_get(ChnotMetadata::ID)?,
            namespace: self.try_get(ChnotMetadata::NAMESPACE)?,
            kind: self.try_get(ChnotMetadata::KIND)?,
            pin_time: self.try_get_df_opt(ChnotMetadata::PIN_TIME)?,
            delete_time: self.try_get_df_opt(ChnotMetadata::DELETE_TIME)?,
            update_time: self.try_get_df_opt(ChnotMetadata::UPDATE_TIME)?,
            insert_time: self.try_get_df(ChnotMetadata::INSERT_TIME)?,
            archive_time: self.try_get_df_opt(ChnotMetadata::ARCHIVE_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.try_get(ChnotRecord::ID)?,
            meta_id: self.try_get(ChnotRecord::META_ID)?,
            content: self.try_get(ChnotRecord::CONTENT)?,
            omit_time: self.try_get_df_opt(ChnotRecord::OMIT_TIME)?,
            insert_time: self.try_get_df(ChnotRecord::INSERT_TIME)?,
        };
        Ok(chnot)
    }

    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: self.try_get(LLMChatBot::ID)?,
            insert_time: self.try_get_df(LLMChatBot::INSERT_TIME)?,
            delete_time: self.try_get_df_opt(LLMChatBot::DELETE_TIME)?,
            name: self.try_get(LLMChatBot::NAME)?,
            body: self.try_get(LLMChatBot::BODY)?,
            update_time: self.try_get_df_opt(LLMChatBot::UPDATE_TIME)?,
            svg_logo: self.try_get(LLMChatBot::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: self.try_get(LLMChatTemplate::ID)?,
            insert_time: self.try_get_df(LLMChatTemplate::INSERT_TIME)?,
            delete_time: self.try_get_df_opt(LLMChatTemplate::DELETE_TIME)?,
            update_time: self.try_get_df_opt(LLMChatTemplate::UPDATE_TIME)?,
            name: self.try_get(LLMChatTemplate::NAME)?,
            prompt: self.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: self.try_get(LLMChatTemplate::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: self.try_get(LLMChatSession::ID)?,
            insert_time: self.try_get_df(LLMChatSession::INSERT_TIME)?,
            template_id: self.try_get(LLMChatSession::TEMPLATE_ID)?,
            title: self.try_get(LLMChatSession::TITLE)?,
            namespace: self.try_get(LLMChatSession::NAMESPACE)?,
            delete_time: self.try_get_df_opt(LLMChatSession::DELETE_TIME)?,
            update_time: self.try_get_df_opt(LLMChatSession::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: self.try_get(LLMChatRecord::ID)?,
            insert_time: self.try_get_df(LLMChatRecord::INSERT_TIME)?,
            session_id: self.try_get(LLMChatRecord::SESSION_ID)?,
            pre_record_id: self.try_get(LLMChatRecord::PRE_RECORD_ID)?,
            content: self.try_get(LLMChatRecord::CONTENT)?,
            role: self.try_get(LLMChatRecord::ROLE)?,
            role_id: self.try_get(LLMChatRecord::ROLE_ID)?,
            omit_time: self.try_get_df_opt(LLMChatRecord::OMIT_TIME)?,
            reasoning_content: self.try_get(LLMChatRecord::REASONING_CONTENT)?,
        };
        Ok(obj)
    }

    fn to_namespace_record(self) -> AResult<NamespaceRecord> {
        let obj = NamespaceRecord {
            id: self.try_get(NamespaceRecord::ID)?,
            insert_time: self.try_get_df(NamespaceRecord::INSERT_TIME)?,
            name: self.try_get(NamespaceRecord::NAME)?,
            delete_time: self.try_get_df_opt(NamespaceRecord::DELETE_TIME)?,
            update_time: self.try_get_df_opt(NamespaceRecord::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_namespace_relation(self) -> AResult<NamespaceRelation> {
        let obj = NamespaceRelation {
            id: self.try_get(NamespaceRelation::ID)?,
            insert_time: self.try_get_df(NamespaceRelation::INSERT_TIME)?,
            delete_time: self.try_get_df_opt(NamespaceRelation::DELETE_TIME)?,
            update_time: self.try_get_df_opt(NamespaceRelation::UPDATE_TIME)?,
            sub_id: self.try_get(NamespaceRelation::SUB_ID)?,
            parent_id: self.try_get(NamespaceRelation::PARENT_ID)?,
        };
        Ok(obj)
    }

    fn to_resource(self) -> AResult<Resource> {
        let obj = Resource {
            id: self.try_get(Resource::ID)?,
            insert_time: self.try_get_df(Resource::INSERT_TIME)?,
            delete_time: self.try_get_df_opt(Resource::DELETE_TIME)?,
            namespace: self.try_get(Resource::NAMESPACE)?,
            ori_filename: self.try_get(Resource::ORI_FILENAME)?,
            content_type: self.try_get(Resource::CONTENT_TYPE)?,
        };
        Ok(obj)
    }

    fn to_kv(self) -> AResult<KV> {
        let obj = KV {
            insert_time: self.try_get_df(KV::INSERT_TIME)?,
            key: self.try_get(KV::KEY)?,
            value: self.try_get(KV::VALUE)?,
            update_time: self.try_get_df_opt(KV::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.try_get(ChnotTag::ID)?,
            namespace: self.try_get(ChnotTag::NAMESPACE)?,
            tag: self.try_get(ChnotTag::TAG)?,
            chnot_meta_id: self.try_get(ChnotTag::CHNOT_META_ID)?,
            insert_time: self.try_get_df(ChnotTag::INSERT_TIME)?,
            category: ChnotTagType::Common,
        };
        Ok(obj)
    }

    fn to_inline_resource(self) -> AResult<crate::model::db::resource::InlineResource> {
        let obj = InlineResource {
            id: self.try_get(InlineResource::ID)?,
            name: self.try_get(InlineResource::NAME)?,
            content: self.try_get(InlineResource::CONTENT)?,
            content_type: self.try_get(InlineResource::CONTENT_TYPE)?,
            delete_time: self.try_get_df_opt(InlineResource::DELETE_TIME)?,
            insert_time: self.try_get_df(InlineResource::INSERT_TIME)?,
            namespace: self.try_get(InlineResource::NAMESPACE)?,
            rid: self.try_get(InlineResource::RID)?,
            archor: self.try_get(InlineResource::ARCHOR)?,
        };
        Ok(obj)
    }
}
