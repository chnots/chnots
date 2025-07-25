use chin_sql::{OnConflict, SqlBuilder, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};
use itertools::Itertools;

use crate::{
    krate::sync::{
        dto::{FetchTIDReq, SyncFetchTIDReq, SyncFetchTIDRsp, SyncInfo},
        mapper::{Dumper, MergableRec, SyncMapper},
        po::SyncLogTransient,
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTx, helper::create_tables,
    },
};

use super::dto::SyncTableEnum;

impl Dumper<KDbRow> for KDb {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchTIDReq,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(KDbRow) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        let FetchTIDReq {
            start_ex,
            end_in,
            page_size,
        } = fetch_data;
        let sql = SqlBuilder::read_all(table_name)
            .r#where(Wheres::and([
                // all table must have tid field
                Wheres::compare("tid", ">", start_ex),
                Wheres::compare("tid", "<=", end_in),
            ]))
            .limit(page_size);

        let rows = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(rows)
    }
}

impl SyncMapper for KDb {
    async fn ensure_sync_table(&self) -> EResult {
        create_tables(vec![SyncLogTransient::create_sql().to_owned_sql()], self).await?;
        Ok(())
    }

    async fn sync_insert_sync_log(&self, log: SyncLogTransient) -> EResult {
        self.conn().await?.exec(log.to_sql_inserter()).await?;
        Ok(())
    }

    async fn sync_get_sync_time(
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

    async fn sync_dump_tids(&self, req: SyncFetchTIDReq) -> AResult<SyncFetchTIDRsp> {
        let start_ex = req.sync_info.start_ex;
        let end_in = req.sync_info.end_in;
        let page_size = req.page_size;
        let table_name = req.sync_info.table;
        let sql = format!(
            "select tid from {} where tid > {} and tid < {} order by tid asc limit {}",
            if req.hist {
                table_name.to_hist_table_name()
            } else {
                table_name.to_table_name()
            },
            start_ex.as_num(),
            end_in.as_num(),
            page_size
        );
        let data = self
            .conn()
            .await?
            .qry_list(sql, |c| c.try_get("tid"))
            .await?;

        Ok(SyncFetchTIDRsp { data })
    }

    async fn sync_merge_tids(
        &self,
        data: SyncFetchTIDRsp,
        hist: bool,
        sync_info: SyncInfo,
    ) -> EResult {
        let tn = sync_info.to_table_name();
        let dtype = if hist { 2 } else { 1 };
        self.conn()
            .await?
            .exec(format!(
                "insert into {}(tid, ltype, rtype) values {} on conflict(tid) do update set rtype = {}",
                tn,
                data.data.iter().map(|t| format!("({}, 0, {})", t.as_num(), dtype)).join(","),
                dtype
            ))
            .await?;
        Ok(())
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
    pub(super) async fn create_tmp_table(&self, sync_info: &SyncInfo) -> EResult {
        let table_name = sync_info.to_table_name();

        let sql = format!(
            "create table if not exists {table_name}(tid bigint, ltype int not null, rtype int not null, primary key (tid))"
        );
        self.conn().await?.exec(sql).await?;
        let sql = format!(
            "insert into {}
            select tid, 1 as ltype, 0 as rtype from {} where tid > {} and tid <= {}
            union
            select tid, 2 as ltype, 0 as rtype from {}_hist where tid > {} and tid <= {}",
            table_name,
            sync_info.table.to_table_name(),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num(),
            sync_info.table.to_table_name(),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num()
        );
        self.conn().await?.exec(sql).await?;
        Ok(())
    }
}

/// Merge records.
/// We have some rules before do the things,
/// 1. all rows cannot to be changed after insertion.
/// 2. the table has a column called `tid` which indicated the insert tid
///    and it's an unique key.
impl KDbTx<'_> {
    pub async fn merge_records<T>(&self, records: Vec<T>, hist: bool) -> AResult<Vec<T>>
    where
        T: Clone + Send + 'static + MergableRec,
    {
        let mut not_same = vec![];
        if hist {
            for rec in records {
                let c = self
                    .exec(
                        rec.to_hist_inserter()
                            .on_conflict(chin_sql::OnConflict::Ignore),
                    )
                    .await?;
                if c > 0 {
                    not_same.push(rec);
                }
            }
        } else {
            for rec in records {
                let hist_row = self
                    .qry_opt(
                        SqlBuilder::read(rec.hist_table_name(), &["1"])
                            .r#where(Wheres::equal("TID", rec.get_tid())),
                        Ok,
                    )
                    .await?;

                // maybe this record is already in the hist table.
                if hist_row.is_some() {
                    continue;
                }

                let ic = self
                    .exec(rec.to_inserter().on_conflict(OnConflict::Ignore))
                    .await?;

                if ic > 0 {
                    continue;
                }
                let row = self
                    .qry_one(
                        SqlBuilder::read_all(rec.table_name()).r#where(rec.pkey_wheres()),
                        Ok,
                        false,
                    )
                    .await?;

                let tid: TID = row.try_get("tid")?;
                let import_tid = rec.get_tid();

                if tid > import_tid {
                    let c = self.exec(rec.to_hist_inserter()).await?;
                    if c > 0 {
                        not_same.push(rec);
                    }
                } else if tid == import_tid {
                    // do nothing
                } else {
                    self.as_executor()
                        .omit_rows(T::main_table_name(), T::all_fields(), rec.pkey_wheres())
                        .await?;
                    let c = self.exec(rec.to_inserter()).await?;
                    if c > 0 {
                        not_same.push(rec);
                    }
                }
            }
        }

        Ok(not_same)
    }
}
