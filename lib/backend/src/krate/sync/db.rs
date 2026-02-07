use anyhow::Context;
use chin_sql::{OnConflict, SqlBuilder, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};
use itertools::Itertools;
use log::{debug, info};

use crate::{
    krate::sync::{
        dto::{
            SyncDataDto, SyncDataOperation, SyncInfo, SyncOtidTIDListRsp, SyncPageInfo,
            SyncTIDListArg, SyncTIDListPage,
        },
        mapper::{Dumper, SyncMapper},
        po::SyncLogTransientCommit,
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
        KDbTransactionBehaiver,
        helper::{Ddls, print_ddls},
    },
    model::OtidTableSupport,
};

use super::po::{RecordState, TidCompare};

const C_TID: &str = "tid";
const C_LSTATE: &str = "lstate";
const C_RSTATE: &str = "rstate";
const C_STATE_CUR: i32 = 1;
const C_STATE_HIST: i32 = 2;
const C_STATE_ABSENT: i32 = 0;
const C_MIN_SYNC: &str = "min_sync";

impl Dumper for KDb {
    async fn dump<E>(&self, fetch_data: SyncTIDListPage, hist: bool) -> chin_tools::AResult<Vec<E>>
    where
        E: OtidTableSupport,
    {
        let sql = match fetch_data {
            SyncTIDListPage::StartEnd {
                start_ex,
                end_in,
                page_size,
            } => {
                SqlBuilder::read_all(E::table_name(hist))
                    .r#where(Wheres::and([
                        // all table must have tid field
                        Wheres::compare(C_TID, ">", start_ex),
                        Wheres::compare(C_TID, "<=", end_in),
                    ]))
                    .limit(page_size)
            }
        };

        let rows = self
            .conn()
            .await?
            .qry_list(sql, |row| E::try_from_kdb_row(&row))
            .await?;

        Ok(rows)
    }
}

impl SyncMapper for KDb {
    async fn ensure_sync_table(&self) -> EResult {
        print_ddls(
            Ddls::new().with_ddl(SyncLogTransientCommit::create_sql().to_owned_sql()),
            self,
        )
        .await?;
        Ok(())
    }

    async fn sync_log_transient_commit(&self, log: SyncLogTransientCommit) -> EResult {
        self.conn().await?.exec(log.to_sql_inserter()).await?;
        Ok(())
    }

    async fn sync_get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        let last_sync = SqlBuilder::read_all(SyncLogTransientCommit::TABLE)
            .r#where(Wheres::and([
                Wheres::equal(SyncLogTransientCommit::TABLE_NAME, table_name.clone()),
                Wheres::equal(SyncLogTransientCommit::REMOTE_ID, remote_id),
            ]))
            .seg("order by")
            .seg(SyncLogTransientCommit::SYNC_FINISH_TID)
            .seg("desc")
            .limit(1);

        let sync: Option<SyncLogTransientCommit> = self
            .conn()
            .await?
            .qry_opt(last_sync, |row| (&row).try_into())
            .await?;

        debug!("last sync log for {table_name} is {sync:?}");

        let Some(sync_log) = sync else {
            return Ok(TID::try_from(0)?);
        };

        let sql = SqlBuilder::read(
            SyncLogTransientCommit::TABLE,
            &[format!(
                "min({}) as {C_MIN_SYNC}",
                SyncLogTransientCommit::START_TID_EX
            )
            .as_str()],
        )
        .r#where(Wheres::and([
            Wheres::equal(SyncLogTransientCommit::TABLE_NAME, table_name.clone()),
            Wheres::compare(
                SyncLogTransientCommit::SYNC_FINISH_TID,
                ">",
                sync_log.sync_finish_tid.as_num(),
            ),
            Wheres::compare(
                SyncLogTransientCommit::START_TID_EX,
                "<",
                sync_log.end_sync_in,
            ),
        ]));

        let s: Option<Option<TID>> = self
            .conn()
            .await?
            .qry_opt(sql, |r| {
                let c: Option<TID> = r.try_get(C_MIN_SYNC)?;
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

    async fn sync_otid_tid_list<T: OtidTableSupport>(
        &self,
        req: SyncTIDListArg<T>,
    ) -> AResult<SyncOtidTIDListRsp> {
        let table_name = T::table_name(req.dto.hist);
        let sql = match req.dto.page {
            super::dto::SyncTIDListPage::StartEnd {
                start_ex,
                end_in,
                page_size,
            } => {
                format!(
                    "select {C_TID} from {table_name} where {C_TID} > {start_ex} and {C_TID} < {end_in} order by {C_TID} asc limit {page_size}"
                )
            }
        };
        let data = self
            .conn()
            .await?
            .qry_list(sql, |c| c.try_get(C_TID))
            .await?;

        Ok(SyncOtidTIDListRsp { data })
    }

    async fn sync_otid_merge_tids<T: OtidTableSupport>(
        &self,
        rsp: SyncOtidTIDListRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult {
        let table_name = sync_info.to_table_name();
        let dtype = if hist {
            RecordState::Hist
        } else {
            RecordState::Cur
        };
        if rsp.data.is_empty() {
            return Ok(());
        }

        let data = rsp
            .data
            .iter()
            .map(|t| format!("({}, 0, {})", t.as_num(), dtype.as_num()))
            .join(",");
        let dtype = dtype.as_num();
        self.conn()
            .await?
            .exec(format!(
                "insert into {table_name}({C_TID}, {C_LSTATE}, {C_RSTATE}) values {data} on conflict({C_TID}) do update set {C_RSTATE} = {dtype}"
            ))
            .await?;
        Ok(())
    }

    async fn sync_fetch_operations<T: OtidTableSupport>(
        &self,
        sync_page: &SyncPageInfo<T>,
    ) -> AResult<SyncDataDto<T>> {
        let tids = self.sync_otid_fetch_tid_compares(sync_page).await?;
        let mut cmds = vec![];
        let mut max_tid = TID::try_from(0).unwrap();

        let nomore = tids.len() < sync_page.page_size;

        let mut hist_pos = vec![];
        let mut cur_tids = vec![];
        let mut to_omit_tids = vec![];

        for tc in tids.iter() {
            let l = tc.lstate;
            let r = tc.rstate;
            let tid = tc.tid;
            if tid > max_tid {
                max_tid = tid;
            }

            if matches!(l, RecordState::Absent) && matches!(r, RecordState::Cur) {
                cmds.push(SyncDataOperation::Pull { tid, hist: false });
            } else if matches!(l, RecordState::Absent) && matches!(r, RecordState::Hist) {
                cmds.push(SyncDataOperation::Pull { tid, hist: true });
            } else if matches!(l, RecordState::Cur) && matches!(r, RecordState::Absent) {
                cur_tids.push(tid);
            } else if matches!(l, RecordState::Hist) && matches!(r, RecordState::Absent) {
                hist_pos.push(tid);
            } else if matches!(l, RecordState::Cur) && matches!(r, RecordState::Hist) {
                to_omit_tids.push(tid);
            } else if matches!(l, RecordState::Hist) && matches!(r, RecordState::Cur) {
                cmds.push(SyncDataOperation::Omit(tid));
            }
        }
        let rec: Vec<T> = self.sync_otid_fetch_records(false, cur_tids).await?;
        cmds.extend(rec.into_iter().map(|e| SyncDataOperation::Push {
            data: e,
            hist: false,
        }));
        let rec = self.sync_otid_fetch_records(true, hist_pos).await?;
        cmds.extend(rec.into_iter().map(|e| SyncDataOperation::Push {
            data: e,
            hist: true,
        }));
        self.conn()
            .await?
            .as_executor()
            .omit_rows::<T>(Wheres::r#in(C_TID, to_omit_tids))
            .await?;

        Ok(SyncDataDto {
            cmds,
            max_tid,
            nomore,
        })
    }

    async fn sync_otid_merge_operations<T: OtidTableSupport>(
        &self,
        req: SyncDataDto<T>,
    ) -> AResult<SyncDataDto<T>> {
        let SyncDataDto {
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
                        .omit_rows::<T>(Wheres::equal(C_TID, tid))
                        .await?;
                }
                SyncDataOperation::Pull { tid, hist } => {
                    let c = self
                        .conn()
                        .await?
                        .qry_opt(
                            format!(
                                "select * from {} where {C_TID} = {tid}",
                                T::table_name(hist)
                            ),
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

        Ok(SyncDataDto {
            cmds: rsp_cmds,
            max_tid: TID::never(),
            nomore: false,
        })
    }

    async fn sync_otid_create_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult {
        let table_name = sync_info.to_table_name();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(format!("drop table if exists {table_name}"))
            .await?;
        let sql = format!(
            "create table {table_name}({C_TID} bigint, {C_LSTATE} int not null, {C_RSTATE} int not null, primary key ({C_TID}))"
        );
        tx.exec(sql).await?;
        let sql = format!(
            "insert into {table_name} select {C_TID}, 1 as {C_LSTATE}, 0 as {C_RSTATE} from {} where {C_TID} > {} and {C_TID} <= {} union select {C_TID}, 2 as {C_LSTATE}, 0 as {C_RSTATE} from {} where {C_TID} > {} and {C_TID} <= {}",
            T::table_name(false),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num(),
            T::table_name(true),
            sync_info.start_ex.as_num(),
            sync_info.end_in.as_num()
        );
        tx.exec(sql).await?;
        tx.cmt().await?;
        Ok(())
    }

    async fn sync_otid_drop_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult {
        let table_name = sync_info.to_table_name();
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(format!("drop table if exists {table_name}"))
            .await?;
        tx.cmt().await?;
        Ok(())
    }

    async fn po_sid_sync_list<const LIMIT: usize, T: crate::model::SidTableSupport>(
        &self,
        sync_info: Vec<Varchar<LIMIT>>,
    ) -> AResult<Vec<T>> {
        self.po_sid_list(sync_info).await
    }

    async fn po_sid_sync_commit<T: crate::model::SidTableSupport>(
        &self,
        sync_info: Vec<T>,
    ) -> EResult {
        self.po_sid_insert(sync_info).await?;
        Ok(())
    }
}

impl TryFrom<&KDbRow> for SyncLogTransientCommit {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let sl = SyncLogTransientCommit {
            remote_id: value.try_get(SyncLogTransientCommit::REMOTE_ID)?,
            table_name: value.try_get(SyncLogTransientCommit::TABLE_NAME)?,
            end_sync_in: value.try_get(SyncLogTransientCommit::END_SYNC_IN)?,
            start_tid_ex: value.try_get(SyncLogTransientCommit::START_TID_EX)?,
            sync_finish_tid: value.try_get(SyncLogTransientCommit::SYNC_FINISH_TID)?,
        };

        Ok(sl)
    }
}

impl KDb {
    async fn sync_otid_fetch_tid_compares<T: OtidTableSupport>(
        &self,
        sync_page: &SyncPageInfo<T>,
    ) -> AResult<Vec<TidCompare>> {
        let tn = sync_page.to_table_name();
        let reader = SqlBuilder::read_all(&tn)
            .r#where(Wheres::and([
                Wheres::compare(C_TID, ">", sync_page.start_ex),
                Wheres::compare_str(C_LSTATE, "<>", C_RSTATE),
                match sync_page.sync_step {
                    super::dto::SyncSingleStep::Omit => Wheres::and([
                        Wheres::compare_str(C_LSTATE, "<>", C_RSTATE),
                        Wheres::r#in(C_LSTATE, [&C_STATE_CUR, &C_STATE_HIST].to_vec()),
                        Wheres::r#in(C_RSTATE, [&C_STATE_CUR, &C_STATE_HIST].to_vec()),
                    ]),
                    super::dto::SyncSingleStep::Data => Wheres::or([
                        Wheres::equal(C_LSTATE, C_STATE_ABSENT),
                        Wheres::equal(C_RSTATE, C_STATE_ABSENT),
                    ]),
                },
            ]))
            .seg("order by tid asc")
            .limit(sync_page.page_size);

        let res = self
            .conn()
            .await?
            .qry_list(reader, |r| {
                Ok(TidCompare {
                    tid: r.try_get(C_TID)?,
                    lstate: {
                        let c: i64 = r.try_get(C_LSTATE)?;
                        let c: i32 = c.clamp(0, 100).try_into()?;
                        c.try_into()?
                    },
                    rstate: {
                        let c: i64 = r.try_get(C_RSTATE)?;
                        let c: i32 = c.clamp(0, 100).try_into()?;
                        c.try_into()?
                    },
                })
            })
            .await?;
        Ok(res)
    }

    async fn sync_otid_fetch_records<T: OtidTableSupport>(
        &self,
        hist: bool,
        tids: Vec<TID>,
    ) -> AResult<Vec<T>> {
        let res = self
            .conn()
            .await?
            .qry_list(
                SqlBuilder::read_all(T::table_name(hist)).r#where(Wheres::r#in(C_TID, tids)),
                move |row| T::try_from_kdb_row(&row),
            )
            .await?;

        Ok(res)
    }

    async fn sync_merge_one_record<T: OtidTableSupport>(&self, data: T, hist: bool) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        // For history table, just ignore the error.
        if hist {
            tx.exec(
                data.sql_inserter()
                    .table_name(T::table_name(hist))
                    .on_conflict(OnConflict::Ignore),
            )
            .await?;
        } else {
            let first_insert = tx
                .exec(
                    data.sql_inserter()
                        .table_name(T::table_name(hist))
                        .on_conflict(OnConflict::Ignore),
                )
                .await?;
            if first_insert == 0 {
                let db_tid: Option<TID> = tx
                    .qry_opt(
                        SqlBuilder::read(T::table_name(false), &[C_TID]).r#where(data.pkey()),
                        |row| row.try_get(C_TID),
                    )
                    .await?;
                let db_tid =
                    db_tid.context("it should be found, maybe some other error.".to_string())?;
                if db_tid > data.tid() {
                    tx.exec(
                        data.sql_inserter()
                            .table_name(T::table_name(hist))
                            .on_conflict(OnConflict::Ignore),
                    )
                    .await?;
                } else if db_tid < data.tid() {
                    tx.as_executor().po_otid_insert([data]).await?;
                }
            }
        }

        tx.cmt().await?;

        Ok(())
    }
}
