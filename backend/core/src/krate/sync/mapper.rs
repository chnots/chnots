use chin_sql::{SqlInserter, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    krate::{
        kkv::{KKVTransient, mapper::KKVMapper},
        sync::{
            dto::FetchDataType,
            po::{SyncAllEndpoints, SyncLogTransient},
        },
    },
    magics::ALL_ENDPOINTS,
    mapper::{MapperRowType, MapperType, db::HistCreateSql},
};

pub trait Dumper<T> {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(T) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static;
}

impl Dumper<MapperRowType> for MapperType {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(MapperRowType) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        match self {
            MapperType::KDb(kdb) => {
                kdb.dump(table_name, fetch_data, move |row| {
                    mapper(MapperRowType::KDb(row))
                })
                .await
            }
        }
    }
}

pub trait SyncMapper {
    async fn ensure_sync_table(&self) -> EResult;
    async fn insert_sync_log(&self, log: SyncLogTransient) -> EResult;
    async fn get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID>;
}

pub trait MergableRec: for<'a> HistCreateSql<'a> {
    fn to_inserter(&self) -> SqlInserter<'static>;
    fn to_hist_inserter(&self) -> SqlInserter<'static>;
    fn pkey_wheres(&self) -> Wheres<'static>;
    fn get_tid(&self) -> TID;
    fn hist_table_name(&self) -> &'static str;
    fn table_name(&self) -> &'static str;
    fn all_fields() -> &'static [&'static str];
}

pub trait SyncOperator<E: MergableRec + Send + 'static> {
    async fn get<F>(
        &self,
        fetch_data: crate::krate::sync::dto::FetchDataType,
        mapper: F,
        hist: bool,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(MapperRowType) -> AResult<E> + Send + Sync + 'static;

    async fn put(&self, recs: Vec<E>, hist: bool) -> AResult<Vec<E>>;
}

impl SyncMapper for MapperType {
    async fn ensure_sync_table(&self) -> EResult {
        expand_mt_branch!(self.ensure_sync_table())
    }

    async fn insert_sync_log(&self, log: SyncLogTransient) -> EResult {
        expand_mt_branch!(self.insert_sync_log(log))
    }

    async fn get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        expand_mt_branch!(self.get_sync_time(table_name, remote_id))
    }
}

impl MapperType {
    pub(crate) async fn get_all_endpoints(&self) -> AResult<SyncAllEndpoints> {
        let kkv: Option<SyncAllEndpoints> = self
            .kkv_transient_query(ALL_ENDPOINTS, |e| Ok(serde_json::from_str(e.as_str())?))
            .await?;
        match kkv {
            Some(kkv) => Ok(kkv),
            None => Ok(SyncAllEndpoints {
                endpoints: Default::default(),
            }),
        }
    }

    pub(crate) async fn overwrite_endpoints(&self, req: SyncAllEndpoints) -> EResult {
        self
            .kkv_transisent_overwrite(
                ALL_ENDPOINTS.to_string().try_into()?,
                &req,
                chin_sql::OnConflict::Replace(KKVTransient::KEY.to_string()),
            )
            .await
    }
}

#[macro_export]
macro_rules! impl_sync_operator {
    ($st:ty, $($field:ident),+) => {

        impl $crate::krate::sync::mapper::MergableRec for $st {
            fn to_inserter(&self) -> chin_sql::SqlInserter<'static> {
                self.clone().to_sql_inserter()
            }

            fn to_hist_inserter(&self) -> chin_sql::SqlInserter<'static> {
                self.clone().to_sql_inserter().table_name(Self::HIST_TABLE)
            }

            fn get_tid(&self) -> TID {
                self.tid
            }

            fn all_fields() -> &'static [&'static str] {
                Self::all_field_names()
            }

            fn pkey_wheres(&self) -> chin_sql::Wheres<'static> {
                Self::pkey_cond($(self.$field.clone()),+)
            }

            #[inline]
            fn hist_table_name(&self) -> &'static str {
                Self::HIST_TABLE
            }

            #[inline]
            fn table_name(&self) -> &'static str {
                Self::TABLE
            }
        }

        impl $crate::krate::sync::mapper::SyncOperator<$st> for $crate::mapper::MapperType {
            async fn get<F>(
                &self,
                fetch_data: $crate::krate::sync::dto::FetchDataType,
                mapper: F,
                hist: bool,
            ) -> chin_tools::AResult<Vec<$st>>
            where
                F: Fn($crate::mapper::MapperRowType) -> chin_tools::AResult<$st>
                    + Send
                    + Sync
                    + 'static,
            {
                use $crate::krate::sync::mapper::Dumper as _;
                let table_name = if hist {
                    <$st>::HIST_TABLE
                } else {
                    <$st>::TABLE
                };
                self.dump(table_name, fetch_data, mapper).await
            }

            async fn put(&self, recs: Vec<$st>, hist: bool) -> chin_tools::AResult<Vec<$st>> {
                use $crate::mapper::db::kdb::KDbBehaiver as _;
                use $crate::mapper::db::kdb::KDbConnBehaiver as _;
                use $crate::mapper::db::kdb::KDbTransactionBehaiver as _;
                let result = match self {
                    $crate::mapper::MapperType::KDb(kdb) => {
                        let mut conn = kdb.conn().await?;
                        let tx = conn.tx().await?;
                        let res = tx.merge_records(recs, hist).await?;
                        tx.cmt().await?;
                        res
                    }
                };

                Ok(result)
            }
        }
    };
}
