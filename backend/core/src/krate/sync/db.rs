use anyhow::Ok;
use chin_sql::{OnConflict, SqlBuilder, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};
use itertools::Itertools;
use log::{debug, info};

use crate::{
    krate::sync::{
        dto::{
            SyncDataArg, SyncDataOperation, SyncFetchTIDArg, SyncFetchTIDPage, SyncFetchTIDRsp,
            SyncInfo, SyncPageInfo,
        },
        mapper::{Dumper, SyncMapper},
        po::SyncLogTransient,
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
        KDbTransactionBehaiver, helper::create_tables,
    },
    model::KOtidSupport,
};

use super::po::{RecordState, TidCompare};

impl Dumper<KDbRow> for KDb {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: SyncFetchTIDPage,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(KDbRow) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        let sql = match fetch_data {
            SyncFetchTIDPage::StartEnd {
                start_ex,
                end_in,
                page_size,
            } => {
                SqlBuilder::read_all(table_name)
                    .r#where(Wheres::and([
                        // all table must have tid field
                        Wheres::compare("tid", ">", start_ex),
                        Wheres::compare("tid", "<=", end_in),
                    ]))
                    .limit(page_size)
            }
        };

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
            .qry_opt(last_sync, |row| (&row).try_into())
            .await?;

        debug!("last sync log for {table_name} is {sync:?}");

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
                ">",
                sync_log.sync_finish_tid.as_num(),
            ),
            Wheres::compare(SyncLogTransient::START_TID_EX, "<", sync_log.end_sync_in),
        ]));

        let s: Option<Option<TID>> = self
            .conn()
            .await?
            .qry_opt(sql, |r| {
                let c: Option<TID> = r.try_get("min_sync")?;
                Ok(c)
            })
            .await?;

        let st = if let Some(Some(s)) = s {
            s
        } else {
            sync_log.end_sync_in
        };

        info!("sync time for {} is {}({})", table_name, st, st.as_utc());

        Ok(st)
    }

    async fn sync_fetch_tids<T: KOtidSupport>(
        &self,
        req: SyncFetchTIDArg<T>,
    ) -> AResult<SyncFetchTIDRsp> {
        let sql = match req.dto.page {
            super::dto::SyncFetchTIDPage::StartEnd {
                start_ex,
                end_in,
                page_size,
            } => {
                format!(
                    "select tid from {} where tid > {} and tid < {} order by tid asc limit {}",
                    T::table_name(req.dto.hist),
                    start_ex.as_num(),
                    end_in.as_num(),
                    page_size
                )
            }
        };
        let data = self
            .conn()
            .await?
            .qry_list(sql, |c| c.try_get("tid"))
            .await?;

        Ok(SyncFetchTIDRsp { data })
    }

    async fn sync_merge_tids<T: KOtidSupport>(
        &self,
        rsp: SyncFetchTIDRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult {
        let tn = sync_info.to_table_name();
        let dtype = if hist {
            RecordState::Hist
        } else {
            RecordState::Cur
        };
        if rsp.data.is_empty() {
            return Ok(());
        }

        self.conn()
            .await?
            .exec(format!(
                "insert into {}(tid, lstate, rstate) values {} on conflict(tid) do update set rstate = {}",
                tn,
                rsp.data.iter().map(|t| format!("({}, 0, {})", t.as_num(), dtype.as_num())).join(","),
                dtype.as_num()
            ))
            .await?;
        Ok(())
    }

    async fn sync_fetch_operations<T: KOtidSupport>(
        &self,
        sync_page: &SyncPageInfo<T>,
    ) -> AResult<SyncDataArg<T>> {
        let tids = self.sync_fetch_tid_compares(sync_page).await?;
        let mut operations = vec![];
        let mut max_tid = TID::from(0);

        let nomore = tids.len() < sync_page.page_size;
        info!("{} : {} -> {}", tids.len(), sync_page.page_size, nomore);

        for tc in tids.iter() {
            let l = tc.lstate;
            let r = tc.rstate;
            let tid = tc.tid;
            if tid > max_tid {
                max_tid = tid;
            }

            if matches!(l, RecordState::Absent) && matches!(r, RecordState::Cur) {
                operations.push(SyncDataOperation::Pull { tid, hist: false });
            } else if matches!(l, RecordState::Absent) && matches!(r, RecordState::Hist) {
                operations.push(SyncDataOperation::Pull { tid, hist: true });
            } else if matches!(l, RecordState::Cur) && matches!(r, RecordState::Absent) {
                let rec = self.sync_fetch_one_record(false, tid).await?;
                if let Some(data) = rec {
                    operations.push(SyncDataOperation::Push { data, hist: false });
                }
            } else if matches!(l, RecordState::Hist) && matches!(r, RecordState::Absent) {
                let rec = self.sync_fetch_one_record(true, tid).await?;
                if let Some(data) = rec {
                    operations.push(SyncDataOperation::Push { data, hist: true });
                }
            } else if matches!(l, RecordState::Cur) && matches!(r, RecordState::Hist) {
                self.conn()
                    .await?
                    .as_executor()
                    .omit_rows::<T>(Wheres::compare("tid", "=", tid))
                    .await?;
            } else if matches!(l, RecordState::Hist) && matches!(r, RecordState::Cur) {
                operations.push(SyncDataOperation::Omit(tid));
            }
        }
        Ok(SyncDataArg {
            cmds: operations,
            max_tid,
            nomore,
        })
    }

    async fn sync_merge_operations<T: KOtidSupport>(
        &self,
        req: SyncDataArg<T>,
    ) -> AResult<SyncDataArg<T>> {
        let SyncDataArg {
            cmds,
            max_tid: _,
            nomore: _,
        } = req;
        let mut rsp_cmds = vec![];
        for ele in cmds {
            match ele {
                SyncDataOperation::Omit(tid) => {
                    self.conn()
                        .await?
                        .as_executor()
                        .omit_rows::<T>(Wheres::compare("tid", "=", tid))
                        .await?;
                }
                SyncDataOperation::Pull { tid, hist } => {
                    let c = self
                        .conn()
                        .await?
                        .qry_opt(
                            format!("select * from {} where tid = {}", T::table_name(hist), tid),
                            move |c| T::try_from_kdb_row(&c),
                        )
                        .await?;
                    if let Some(data) = c {
                        rsp_cmds.push(SyncDataOperation::Push { data, hist });
                    }
                }
                SyncDataOperation::Push { data, hist } => {
                    self.sync_merge_one_record(data, hist).await?;
                }
            }
        }

        Ok(SyncDataArg {
            cmds: rsp_cmds,
            max_tid: TID::never(),
            nomore: false,
        })
    }

    async fn sync_create_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult {
        let table_name = sync_info.to_table_name();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(format!("drop table if exists {table_name}"))
            .await?;
        let sql = format!(
            "create table  {table_name}(tid bigint, lstate int not null, rstate int not null, primary key (tid))"
        );
        tx.exec(sql).await?;
        let sql = format!(
            "insert into {}
            select tid, 1 as lstate, 0 as rstate from {} where tid > {} and tid <= {}
            union
            select tid, 2 as lstate, 0 as rstate from {}_hist where tid > {} and tid <= {}",
            table_name,
            T::table_name(false),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num(),
            T::table_name(false),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num()
        );
        tx.exec(sql).await?;
        tx.cmt().await?;
        Ok(())
    }

    async fn sync_drop_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult {
        let table_name = sync_info.to_table_name();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(format!("drop table if exists {table_name}"))
            .await?;
        tx.cmt().await?;
        Ok(())
    }
}

impl TryFrom<&KDbRow> for SyncLogTransient {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
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
    async fn sync_fetch_tid_compares<T: KOtidSupport>(
        &self,
        sync_page: &SyncPageInfo<T>,
    ) -> AResult<Vec<TidCompare>> {
        let tn = sync_page.to_table_name();
        let reader = SqlBuilder::read_all(&tn)
            .r#where(Wheres::and([
                Wheres::compare("tid", ">", sync_page.start_ex),
                Wheres::compare_str("lstate", "<>", "rstate"),
            ]))
            .sov("order by tid asc")
            .limit(sync_page.page_size);

        let res = self
            .conn()
            .await?
            .qry_list(reader, |r| {
                Ok(TidCompare {
                    tid: r.try_get("tid")?,
                    lstate: {
                        let c: i32 = r.try_get("lstate")?;
                        c.try_into()?
                    },
                    rstate: {
                        let c: i32 = r.try_get("rstate")?;
                        c.try_into()?
                    },
                })
            })
            .await?;
        Ok(res)
    }

    async fn sync_fetch_one_record<T: KOtidSupport>(
        &self,
        hist: bool,
        tid: TID,
    ) -> AResult<Option<T>> {
        let res = self
            .conn()
            .await?
            .qry_opt(
                format!("select * from {} where tid = {}", T::table_name(hist), tid),
                move |row| Ok(T::try_from_kdb_row(&row)?),
            )
            .await?;

        Ok(res)
    }

    async fn sync_merge_one_record<T: KOtidSupport>(&self, data: T, hist: bool) -> EResult {
        self.conn()
            .await?
            .exec(
                data.sql_inserter()
                    .table_name(T::table_name(hist))
                    .on_conflict(OnConflict::Ignore),
            )
            .await?;

        Ok(())
    }
}
