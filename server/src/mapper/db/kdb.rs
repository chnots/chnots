use chin_sql::{DateFixedOffset, DbType, IntoSqlSeg, SqlReader};
use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset};
use deadpool_postgres::Client;
use deadpool_sqlite::rusqlite;

use super::{postgres, sqlite};
use crate::mapper::DeserializeMapper;
use crate::model::db::{
    chnot::*,
    llmchat::*,
    workspace::*,
    kfile::KTV,
    kfile::{InlineKFile, KFile},
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

pub trait KDbConnBehaiverSync {
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

pub trait KDbRowBehavier<'b, T> {
    fn try_get(&'b self, key: &str) -> AResult<T>;
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

impl<'a> DeserializeMapper for KDbRow<'a> {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.try_get(ChnotMetadata::ID)?,
            workspace: self.try_get(ChnotMetadata::WORKSPACE)?,
            kind: self.try_get(ChnotMetadata::KIND)?,
            pin_time: self.try_get(ChnotMetadata::PIN_TIME)?,
            delete_time: self.try_get(ChnotMetadata::DELETE_TIME)?,
            update_time: self.try_get(ChnotMetadata::UPDATE_TIME)?,
            insert_time: self.try_get(ChnotMetadata::INSERT_TIME)?,
            archive_time: self.try_get(ChnotMetadata::ARCHIVE_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.try_get(ChnotRecord::ID)?,
            meta_id: self.try_get(ChnotRecord::META_ID)?,
            content: self.try_get(ChnotRecord::CONTENT)?,
            omit_time: self.try_get(ChnotRecord::OMIT_TIME)?,
            insert_time: self.try_get(ChnotRecord::INSERT_TIME)?,
        };
        Ok(chnot)
    }

    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: self.try_get(LLMChatBot::ID)?,
            insert_time: self.try_get(LLMChatBot::INSERT_TIME)?,
            delete_time: self.try_get(LLMChatBot::DELETE_TIME)?,
            name: self.try_get(LLMChatBot::NAME)?,
            body: self.try_get(LLMChatBot::BODY)?,
            update_time: self.try_get(LLMChatBot::UPDATE_TIME)?,
            svg_logo: self.try_get(LLMChatBot::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: self.try_get(LLMChatTemplate::ID)?,
            insert_time: self.try_get(LLMChatTemplate::INSERT_TIME)?,
            delete_time: self.try_get(LLMChatTemplate::DELETE_TIME)?,
            update_time: self.try_get(LLMChatTemplate::UPDATE_TIME)?,
            name: self.try_get(LLMChatTemplate::NAME)?,
            prompt: self.try_get(LLMChatTemplate::PROMPT)?,
            svg_logo: self.try_get(LLMChatTemplate::SVG_LOGO)?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: self.try_get(LLMChatSession::ID)?,
            insert_time: self.try_get(LLMChatSession::INSERT_TIME)?,
            template_id: self.try_get(LLMChatSession::TEMPLATE_ID)?,
            title: self.try_get(LLMChatSession::TITLE)?,
            workspace: self.try_get(LLMChatSession::WORKSPACE)?,
            delete_time: self.try_get(LLMChatSession::DELETE_TIME)?,
            update_time: self.try_get(LLMChatSession::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: self.try_get(LLMChatRecord::ID)?,
            insert_time: self.try_get(LLMChatRecord::INSERT_TIME)?,
            session_id: self.try_get(LLMChatRecord::SESSION_ID)?,
            pre_record_id: self.try_get(LLMChatRecord::PRE_RECORD_ID)?,
            content: self.try_get(LLMChatRecord::CONTENT)?,
            role: self.try_get(LLMChatRecord::ROLE)?,
            role_id: self.try_get(LLMChatRecord::ROLE_ID)?,
            omit_time: self.try_get(LLMChatRecord::OMIT_TIME)?,
            reasoning_content: self.try_get(LLMChatRecord::REASONING_CONTENT)?,
        };
        Ok(obj)
    }

    fn to_workspace_record(self) -> AResult<WorkspaceRecord> {
        let obj = WorkspaceRecord {
            id: self.try_get(WorkspaceRecord::ID)?,
            insert_time: self.try_get(WorkspaceRecord::INSERT_TIME)?,
            name: self.try_get(WorkspaceRecord::NAME)?,
            delete_time: self.try_get(WorkspaceRecord::DELETE_TIME)?,
            update_time: self.try_get(WorkspaceRecord::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_workspace_relation(self) -> AResult<WorkspaceRelation> {
        let obj = WorkspaceRelation {
            id: self.try_get(WorkspaceRelation::ID)?,
            insert_time: self.try_get(WorkspaceRelation::INSERT_TIME)?,
            delete_time: self.try_get(WorkspaceRelation::DELETE_TIME)?,
            update_time: self.try_get(WorkspaceRelation::UPDATE_TIME)?,
            sub_id: self.try_get(WorkspaceRelation::SUB_ID)?,
            parent_id: self.try_get(WorkspaceRelation::PARENT_ID)?,
        };
        Ok(obj)
    }

    fn to_kfile(self) -> AResult<KFile> {
        let obj = KFile {
            id: self.try_get(KFile::ID)?,
            insert_time: self.try_get(KFile::INSERT_TIME)?,
            delete_time: self.try_get(KFile::DELETE_TIME)?,
            workspace: self.try_get(KFile::WORKSPACE)?,
            ori_filename: self.try_get(KFile::ORI_FILENAME)?,
            content_type: self.try_get(KFile::CONTENT_TYPE)?,
            ori_last_modified: self.try_get(KFile::ORI_LAST_MODIFIED)?,
            filesize: self.try_get(KFile::FILESIZE)?,
        };
        Ok(obj)
    }

    fn to_kv(self) -> AResult<KTV> {
        let obj = KTV {
            insert_time: self.try_get(KTV::INSERT_TIME)?,
            key: self.try_get(KTV::KEY)?,
            value: self.try_get(KTV::VALUE)?,
            ttype: self.try_get(KTV::TTYPE)?,
            update_time: self.try_get(KTV::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.try_get(ChnotTag::ID)?,
            workspace: self.try_get(ChnotTag::WORKSPACE)?,
            tag: self.try_get(ChnotTag::TAG)?,
            chnot_meta_id: self.try_get(ChnotTag::CHNOT_META_ID)?,
            insert_time: self.try_get(ChnotTag::INSERT_TIME)?,
            category: ChnotTagType::Common,
        };
        Ok(obj)
    }

    fn to_inline_kfile(self) -> AResult<crate::model::db::kfile::InlineKFile> {
        let obj = InlineKFile {
            id: self.try_get(InlineKFile::ID)?,
            name: self.try_get(InlineKFile::NAME)?,
            content: self.try_get(InlineKFile::CONTENT)?,
            content_type: self.try_get(InlineKFile::CONTENT_TYPE)?,
            delete_time: self.try_get(InlineKFile::DELETE_TIME)?,
            insert_time: self.try_get(InlineKFile::INSERT_TIME)?,
            workspace: self.try_get(InlineKFile::WORKSPACE)?,
            rid: self.try_get(InlineKFile::RID)?,
            archor: self.try_get(InlineKFile::ARCHOR)?,
        };
        Ok(obj)
    }
}
