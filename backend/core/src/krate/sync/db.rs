use chin_sql::{str_type::Varchar, time_type::TID, SqlBuilder, SqlInserter, Wheres};
use chin_tools::{AResult, EResult};

use crate::{
    krate::sync::{
        dto::FetchDataType,
        mapper::{Dumper, SyncMapper},
        po::SyncLogTransient,
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, helper::create_tables,
    },
};

impl Dumper<KDbRow> for KDb {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(KDbRow) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        let sql = SqlBuilder::read_all(table_name);
        let sql = match fetch_data {
            FetchDataType::RangePage {
                start_ex,
                end_in,
                page_size,
            } => {
                sql.r#where(Wheres::and([
                    // all table must have tid field
                    Wheres::compare("tid", ">", start_ex),
                    Wheres::compare("tid", "<=", end_in),
                ]))
                .limit(page_size)
            }
            FetchDataType::Tids(tids) => sql.r#where(Wheres::r#in("tid", tids)),
        };

        let rows = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(rows)
    }
}

impl SyncMapper for KDb {
    async fn ensure_sync_table(&self) -> EResult {
        create_tables(vec![SyncLogTransient::create_sql()], self).await?;
        Ok(())
    }

    async fn insert_sync_log(&self, log: SyncLogTransient) -> EResult {
        self.conn().await?.exec(log.to_sql_inserter()).await?;
        Ok(())
    }

    async fn get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        let last_sync = SqlBuilder::read_all(SyncLogTransient::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(SyncLogTransient::TABLE_NAME, table_name.clone()),
                Wheres::equal(SyncLogTransient::REMOTE_ID, remote_id),
            ]))
            .sov("order by")
            .sov(SyncLogTransient::SYNC_FINISH_TID)
            .sov("desc")
            .limit(1);
        let sync: Option<SyncLogTransient> = self
            .conn()
            .await?
            .qry_opt(last_sync, SyncLogTransient::try_from)
            .await?;

        let Some(sync_log) = sync else {
            return Ok(TID::from(0));
        };

        let sql = SqlBuilder::read(
            SyncLogTransient::TABLE,
            &[format!("min({}) as min_sync", SyncLogTransient::START_TID_EX).as_str()],
        )
        .r#where(Wheres::and([
            Wheres::equal(SyncLogTransient::TABLE_NAME, table_name.clone()),
            Wheres::compare(
                SyncLogTransient::SYNC_FINISH_TID,
                ">=",
                sync_log.sync_finish_tid.as_num(),
            ),
            Wheres::compare(SyncLogTransient::START_TID_EX, "<", sync_log.end_sync_in),
        ]));

        let s: Option<TID> = self
            .conn()
            .await?
            .qry_opt(sql, |r| r.try_get("min_sync"))
            .await?;
        let st = if let Some(s) = s {
            s
        } else {
            sync_log.end_sync_in
        };

        Ok(st)
    }
}

impl TryFrom<KDbRow> for SyncLogTransient {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let sl = SyncLogTransient {
            remote_id: value.try_get(SyncLogTransient::REMOTE_ID)?,
            table_name: value.try_get(SyncLogTransient::TABLE_NAME)?,
            end_sync_in: value.try_get(SyncLogTransient::END_SYNC_IN)?,
            start_tid_ex: value.try_get(SyncLogTransient::START_TID_EX)?,
            sync_finish_tid: value.try_get(SyncLogTransient::SYNC_FINISH_TID)?,
        };

        Ok(sl)
    }
}


impl KDb {
    pub async fn merge_into_table<'b, T>(&self, table_name: String, records: Vec<T>) -> EResult where T: Into<SqlInserter<'b>> {
        for rec in records {
            self.conn().await?.exec(rec.into()).await?;
        }

        Ok(())
    }
}