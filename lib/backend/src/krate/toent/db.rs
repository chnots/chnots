use std::collections::HashMap;

use anyhow::bail;
use chin_sql::{
    Froms, GroupBy, ILikeType, JoinCond, JoinTable, Joins, LimitOffset, OrderBy, SqlField,
    SqlReader, SqlTable, Wheres,
    str_type::{Text, Varchar},
    time_type::TID,
};
use chin_tools::AResult;
use itertools::Itertools;

use crate::{
    krate::{
        chnot::ChnotMetaTable,
        mdwt::{MdwtRecord, MdwtRecordTable, db::MdwtOtidInTags},
        toent::{
            ToentDefiCommitReq, ToentInstCommitReq, ToentInstCommitRsp, ToentInstCountReq,
            ToentInstCountRsp, ToentSearchReq, ToentSearchRsp, ToentSearchRspData,
            mapper::ToentMapper,
            po::{ToentDefi, ToentDefiTable, ToentInst, ToentInstTable},
            timeevent::timeenum::UtcWithOffset,
            todoevent::TodoEvent,
        },
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRowBehavier,
        KDbTransactionBehaiver, KDbTx,
    },
    model::{KSerde, dto::KReq},
};

use crate::krate::toent::logic::todoevent::TodoStateEnum;

impl ToentMapper for KDb {
    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp> {
        let mut totals: HashMap<String, usize> = HashMap::new();

        let base_req = ToentSearchReq {
            start_tid: req.start_tid,
            end_tid: req.end_tid,
            include_completed: req.include_completed,
            include_uncompleted: req.include_uncompleted,
            include_no_time_todo: req.include_no_time_todo,
            query: None,
            tags: None,
            start_index: 0,
            page_size: 500,
            chnot_otids: None,
        };
        let total = self.count_search_total(req.frame(base_req)).await?;

        for range in &req.ranges {
            let count_req = ToentSearchReq {
                start_tid: range.start_tid,
                end_tid: range.end_tid,
                include_completed: req.include_completed,
                include_uncompleted: req.include_uncompleted,
                include_no_time_todo: range.include_no_time_todo,
                query: None,
                tags: None,
                start_index: 0,
                page_size: 500,
                chnot_otids: None,
            };
            let count = self.count_search_total(req.frame(count_req)).await?;
            totals.insert(range.key.clone(), count);
        }

        Ok(ToentInstCountRsp { total, totals })
    }

    async fn toent_search(
        &self,
        req: ToentSearchReq,
        spaces: Vec<Varchar<40>>,
    ) -> AResult<ToentSearchRsp> {
        // If both switches are off, caller explicitly asks for no todo states.
        if !req.include_completed && !req.include_uncompleted {
            return Ok(ToentSearchRsp {
                items: vec![],
                has_next: false,
                next_start: req.start_index,
            });
        }

        let tags_req = req.tags.clone();

        let tag_otids = MdwtOtidInTags::new("tag_otids");
        let tags = tag_otids.sub_query_table(tags_req);
        let mr = MdwtRecordTable::new("mr");
        let cm = ChnotMetaTable::new("cm");
        let td = ToentDefiTable::new("td");
        struct EventDefiAndContent {
            content: Text,
            toent_defi: ToentDefi,
        }

        // Base search joins meta + mdwt + toent definition, then optional tag subquery filtering.
        let joins = Joins::new((&cm).into())
            .join(JoinTable {
                join_type: chin_sql::JoinType::LeftJoin,
                table: (&mr).into(),
                conds: [(cm.otid(), mr.otid()).into()].into(),
            })
            .join(JoinTable {
                join_type: chin_sql::JoinType::LeftJoin,
                table: (&td).into(),
                conds: [(cm.otid(), td.otid()).into()].into(),
            })
            .join_some(tags, |tags| JoinTable {
                join_type: chin_sql::JoinType::InnerJoin,
                table: Froms::SubQuery {
                    table: tags.into(),
                    alias: &tag_otids.alias,
                },
                conds: [(cm.otid(), tag_otids.mdwt_otid()).into()].into(),
            });

        let sr = SqlReader::read((mr.content(), td.all_fields()), joins)
            .wheres(Wheres::and([
                Wheres::or([
                    Wheres::or([
                        td.end_tid().v_cmp(">", req.start_tid),
                        td.start_tid().v_cmp("<", req.end_tid),
                    ]),
                    Wheres::if_some(req.include_no_time_todo.then_some(()), |_| {
                        Wheres::and([
                            td.todo_state().v_is_not_null(),
                            Wheres::or([
                                Wheres::and([td.start_tid().v_is_null(), td.end_tid().v_is_null()]),
                                Wheres::and([
                                    td.start_tid().v_eq(TID::never()),
                                    td.end_tid().v_eq(TID::never()),
                                ]),
                            ]),
                        ])
                    }),
                ]),
                Wheres::if_some(req.query.clone(), |e| {
                    mr.content().v_ilike(e, ILikeType::Fuzzy)
                }),
                Wheres::if_when(spaces.len() > 0, cm.kspace().v_in(spaces)),
                td.otid().v_is_not_null(),
                mr.content().v_is_not_null(),
            ]))
            .limit(LimitOffset::new(req.page_size + 1).offset(req.start_index))
            .build2();

        let mut defis: Vec<EventDefiAndContent> = self
            .conn()
            .await?
            .qry_list(sr, |row| {
                let content: Text = row.try_get(MdwtRecord::CONTENT)?;
                let toent_defi: ToentDefi = ToentDefi::try_from_kdb_row(&row)?;

                Ok(EventDefiAndContent {
                    content,
                    toent_defi,
                })
            })
            .await?;

        // Use page_size + 1 to detect "has_next" without an extra count query.
        let has_next = defis.len() > req.page_size;
        if has_next {
            defis.truncate(req.page_size);
        }
        let next_start = req.start_index + defis.len();

        if defis.is_empty() {
            return Ok(ToentSearchRsp {
                next_start,
                items: vec![],
                has_next,
            });
        }

        let otids: Vec<TID> = defis.iter().map(|e| e.toent_defi.otid).collect();
        let ti = ToentInstTable::new("ti");
        // Fetch persisted instances in the requested time window for current page otids.
        let ti_sql = SqlReader::read(ti.all_fields(), &ti)
            .wheres(Wheres::and([
                ti.chnot_otid().v_in(otids),
                Wheres::or([
                    Wheres::and([
                        ti.end_tid().v_cmp(">", req.start_tid),
                        ti.start_tid().v_cmp("<", req.end_tid),
                    ]),
                    if req.include_no_time_todo {
                        ti.start_tid().v_is_null()
                    } else {
                        Wheres::None
                    },
                ]),
            ]))
            .build2();
        let conn = self.conn().await?;

        let db_insts: Vec<ToentInst> = conn
            .qry_list(ti_sql, |row| ToentInst::try_from_kdb_row(&row))
            .await?;

        let otid_name = ti.chnot_otid().twn();
        let tid_name = ti.tid().twn();

        let last_finished_utcs: HashMap<TID, TID> = conn
            .qry_list(
                SqlReader::read(
                    ti.all_fields(),
                    Joins::new((&ti).into()).join(JoinTable {
                        join_type: chin_sql::JoinType::RightJoin,
                        table: Froms::SubQuery {
                            table: SqlReader::read(
                                SqlField {
                                    alias: Some("max_tid"),
                                    inner: chin_sql::SqlFieldInner::Raw {
                                        expr: format!("max({})", tid_name).into(),
                                    },
                                },
                                &ti,
                            )
                            .wheres(Wheres::and([
                                ti.chnot_otid()
                                    .v_in(defis.iter().map(|e| e.toent_defi.otid).collect_vec()),
                                Wheres::equal(
                                    ti.todo_state().twn(),
                                    TodoStateEnum::Done.as_static_str(),
                                ),
                            ]))
                            .group_by(GroupBy::Plain(vec![ti.chnot_otid().twn()]))
                            .build2()
                            .into(),
                            alias: "mtid",
                        },
                        conds: [JoinCond {
                            l_table: "mtid",
                            l_field: "max_tid",
                            r_table: "ti",
                            r_field: "tid",
                        }]
                        .into(),
                    }),
                )
                .build2(),
                move |row| {
                    let cotid: TID = row.try_get(&otid_name)?;
                    let tid: TID = row.try_get(&tid_name)?;
                    Ok((cotid, tid))
                },
            )
            .await?
            .into_iter()
            .map(|(otid, tid)| (otid, tid))
            .collect();

        let mut items: Vec<ToentSearchRspData> = vec![];
        for defi in defis {
            let mut hists: HashMap<Option<TID>, ToentInst> = db_insts
                .iter()
                .filter(|e| e.chnot_otid == defi.toent_defi.otid)
                .map(|e| (e.start_tid, e.clone()))
                .collect();
            let title = first_non_empty_line(defi.content.as_str());
            let last_finished_time = last_finished_utcs
                .get(&defi.toent_defi.otid)
                .map(|e| (*e).into());

            if req.include_uncompleted {
                if let Some(ted) = &defi.toent_defi.event_defi {
                    // Generate predicted upcoming instances from recurrence definition.
                    let generated = ted.generate(
                        defi.toent_defi.otid,
                        defi.toent_defi.todo_state.is_some(),
                        defi.toent_defi.todo_priority,
                        999,
                        last_finished_time,
                        Some(req.start_tid.into()),
                        Some(req.end_tid.into()),
                    )?;
                    for ele in generated.into_iter() {
                        // Keep persisted records when keys collide; generated data fills missing slots only.
                        hists.entry(ele.start_tid).or_insert(ele);
                    }
                }
            }
            hists.retain(|_, inst| match_toent_filters(inst, &req));

            items.push(ToentSearchRspData {
                inst: hists
                    .values()
                    .map(|e| e.to_owned().to_owned())
                    .sorted_by(|e1, e2| e1.start_tid.cmp(&e2.start_tid))
                    .collect(),
                defi: defi.toent_defi.event_defi.clone(),
                title: title.clone(),
            });
        }

        Ok(ToentSearchRsp {
            next_start,
            items,
            has_next,
        })
    }

    async fn toent_inst_commit(
        &self,
        _req: KReq<ToentInstCommitReq>,
    ) -> AResult<ToentInstCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        let rsp = tx.toent_inst_commit(&_req.body).await?;
        tx.cmt().await?;
        Ok(rsp)
    }
}

impl KDb {
    async fn count_search_total(&self, req: KReq<ToentSearchReq>) -> AResult<usize> {
        let mut start_index = req.start_index;
        let mut total: usize = 0;
        let mut has_next = true;
        let mut guard = 0usize;

        while has_next {
            guard += 1;
            if guard > 2000 {
                bail!("toent_inst_count exceeded max paging window");
            }

            let search_req = ToentSearchReq {
                start_tid: req.start_tid,
                end_tid: req.end_tid,
                include_completed: req.include_completed,
                include_uncompleted: req.include_uncompleted,
                include_no_time_todo: req.include_no_time_todo,
                query: req.query.clone(),
                tags: req.tags.clone(),
                start_index,
                page_size: req.page_size.max(1),
                chnot_otids: None,
            };

            let rsp = self.toent_search(search_req, req.get_spaces()).await?;
            total = total.saturating_add(rsp.items.len());
            has_next = rsp.has_next;

            if !has_next {
                break;
            }

            if rsp.next_start <= start_index {
                break;
            }
            start_index = rsp.next_start;
        }

        Ok(total)
    }
}

impl<'a> KDbTx<'a> {
    pub(crate) async fn toent_defi_commit(&self, req: &ToentDefiCommitReq) -> AResult<bool> {
        let ToentDefiCommitReq {
            otid,
            todo_event,
            time_event_field,
        } = &req;
        let otid = *otid;
        let td = ToentDefiTable::new("td");
        let saved_defi: Option<ToentDefi> = self
            .qry_opt(
                SqlReader::read(td.all_fields(), &td)
                    .wheres(td.otid().v_eq(otid))
                    .build2(),
                |row| ToentDefi::try_from_kdb_row(&row),
            )
            .await?;

        let todo_priority = todo_event.and_then(|e| e.priority);
        let to_save_defi = match time_event_field {
            Some(defi) if !defi.time_events.is_empty() => {
                Some(defi.to_po(otid, todo_event.as_ref().map(|e| e.state), todo_priority))
            }
            _ => todo_event
                .as_ref()
                .and_then(|te| ToentDefi::from_todo(otid, te.state, todo_priority)),
        };
        let mut flag = true;
        match (&saved_defi, &to_save_defi) {
            (Some(saved), Some(next))
                if saved.event_defi != next.event_defi || saved.todo_state != next.todo_state =>
            {
                self.po_otid_commit([next.clone()]).await?;
            }
            (None, Some(next)) => {
                self.po_otid_commit([next.clone()]).await?;
            }
            (Some(_), None) => {
                self.omit_rows::<ToentDefi>(Wheres::equal(ToentDefi::OTID, otid))
                    .await?;
            }
            _ => {
                flag = false;
            }
        }
        Ok(flag)
    }

    pub(crate) async fn toent_inst_commit(
        &self,
        req: &ToentInstCommitReq,
    ) -> AResult<ToentInstCommitRsp> {
        let chnot_otid = req.chnot_otid;
        let td = ToentDefiTable::new("td");
        let toent_defi: Option<ToentDefi> = self
            .qry_opt(
                SqlReader::read(td.all_fields(), &td)
                    .wheres(td.otid().v_eq(chnot_otid))
                    .build2(),
                |e| ToentDefi::try_from_kdb_row(&e),
            )
            .await?;

        let ti = ToentInstTable::new("ti");
        let last_finished_inst: Option<ToentInst> = self
            .qry_opt(
                SqlReader::read(ti.all_fields(), &ti)
                    .wheres(Wheres::and([
                        ti.chnot_otid().v_eq(chnot_otid),
                        Wheres::equal(ti.todo_state().twn(), TodoStateEnum::Done.as_static_str()),
                    ]))
                    .order_by([OrderBy::Desc(ti.tid().twn())])
                    .limit(LimitOffset::new(1))
                    .build2(),
                |row| ToentInst::try_from_kdb_row(&row),
            )
            .await?;
        let finished_count = last_finished_inst
            .as_ref()
            .map(|e| e.finished_count)
            .unwrap_or_default();
        let last_finished_utc: Option<UtcWithOffset> =
            last_finished_inst.as_ref().map(|e| e.tid).map(|e| e.into());

        let s0_inst = self
            .qry_opt(
                SqlReader::read(ti.all_fields(), &ti)
                    .wheres(ti.otid().v_eq(req.otid))
                    .build2(),
                |row| ToentInst::try_from_kdb_row(&row),
            )
            .await?;

        let s0_state = s0_inst
            .as_ref()
            .and_then(|inst| inst.todo_state)
            .or(toent_defi.as_ref().and_then(|defi| defi.todo_state));

        // Request can omit todo_state; in that case keep previous state and only update note.
        let s1_state = req.todo_state;

        // Nothing to persist if there is no state and note is empty.
        if s1_state.is_none() && req.note.as_str().trim().is_empty() {
            return Ok(ToentInstCommitRsp::default());
        }

        // Build target instance in priority order:
        // 1) mutate existing instance for this inst_tid,
        // 2) derive from event definition,
        // 3) fallback to pure todo instance.
        let mut s1_inst = if let Some(s0_inst) = s0_inst.clone() {
            ToentInst {
                todo_state: s1_state,
                note: Some(req.note.clone()),
                tid: TID::now(),
                ..s0_inst
            }
        } else if let Some(defi) = toent_defi.clone() {
            if let Some(ted) = defi.event_defi {
                let mut generated = ted.generate(
                    chnot_otid,
                    true,
                    defi.todo_priority,
                    1,
                    last_finished_utc.into(),
                    Some(req.start_tid.into()),
                    None,
                )?;
                let Some(mut generated) = generated.pop() else {
                    return Ok(ToentInstCommitRsp::default());
                };
                generated.todo_state = s1_state;
                generated.note = Some(req.note.clone());
                generated
            } else {
                let Some(state) = s1_state else {
                    return Ok(ToentInstCommitRsp::default());
                };
                let mut inst = ToentInst::pure_todo(
                    chnot_otid,
                    TodoEvent {
                        state,
                        priority: defi.todo_priority,
                    },
                    finished_count,
                );
                inst.note = Some(req.note.clone());
                inst
            }
        } else {
            let Some(state) = s1_state else {
                return Ok(ToentInstCommitRsp::default());
            };
            let mut inst = ToentInst::pure_todo(
                chnot_otid,
                TodoEvent {
                    state,
                    priority: None,
                },
                finished_count,
            );
            inst.note = Some(req.note.clone());
            inst
        };

        // finished_count only increases on transition into Done.
        let finished_count = calc_next_finished_count(s0_state, s1_state, finished_count);
        s1_inst.finished_count = finished_count;
        s1_inst.tid = TID::now();

        // Avoid useless writes to keep history cleaner and reduce sync noise.
        if let Some(s0_inst) = &s0_inst {
            if s0_inst == &s1_inst {
                return Ok(ToentInstCommitRsp::default());
            }
        }

        let mut updated_insts = vec![];
        // Persist only meaningful todo/time instances.
        if s1_inst.todo_state.is_some() || s1_inst.start_tid.is_some() || s1_inst.end_tid.is_some()
        {
            self.po_otid_commit([s1_inst.clone()]).await?;
            updated_insts.push(s1_inst.clone());
        }

        if s1_inst.todo_state.is_some_and(|s| s == TodoStateEnum::Done) {
            if let Some(defi) = toent_defi.clone() {
                if let Some(ted) = defi.event_defi {
                    let mut generated = ted.generate(
                        chnot_otid,
                        true,
                        defi.todo_priority,
                        1,
                        last_finished_utc,
                        Some(req.start_tid.into()),
                        None,
                    )?;
                    if let Some(mut s2_inst) = generated.pop() {
                        s2_inst.todo_state = TodoStateEnum::Todo.into();
                        s2_inst.note = None;
                        updated_insts.push(s2_inst);
                    };
                }
            }
        }

        Ok(ToentInstCommitRsp { updated_insts })
    }
}

fn match_toent_filters(inst: &ToentInst, req: &ToentSearchReq) -> bool {
    if !req.include_no_time_todo && inst.start_tid.is_none() {
        return false;
    }

    let is_done = matches!(inst.todo_state, Some(TodoStateEnum::Done));
    if is_done && !req.include_completed {
        return false;
    }
    if !is_done && !req.include_uncompleted {
        return false;
    }

    true
}

fn calc_next_finished_count(
    prev_state: Option<TodoStateEnum>,
    next_state: Option<TodoStateEnum>,
    latest_finished_count: i64,
) -> i64 {
    if !matches!(prev_state, Some(TodoStateEnum::Done))
        && matches!(next_state, Some(TodoStateEnum::Done))
    {
        return latest_finished_count + 1;
    }

    latest_finished_count
}

fn first_non_empty_line(content: &str) -> String {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .unwrap_or_default()
}

mod toent_db {
    use crate::{
        krate::toent::po::{ToentDefi, ToentInst},
        krate::toent::{ToentSearchReq, logic::todoevent::TodoStateEnum},
        mapper::db::HistCreateSql,
    };
    use chin_sql::time_type::TID;

    use super::{calc_next_finished_count, match_toent_filters};

    #[test]
    fn test_print_ddls() {
        for ele in ToentInst::ddls() {
            for ele in ele.sqls(chin_sql::DbType::Postgres).unwrap() {
                println!("{};", ele)
            }
        }

        for ele in ToentDefi::ddls() {
            for ele in ele.sqls(chin_sql::DbType::Postgres).unwrap() {
                println!("{};", ele)
            }
        }
    }

    #[test]
    fn test_calc_next_finished_count() {
        assert_eq!(
            calc_next_finished_count(None, Some(TodoStateEnum::Done), 0),
            1
        );
        assert_eq!(
            calc_next_finished_count(Some(TodoStateEnum::Wait), Some(TodoStateEnum::Done), 3),
            4
        );
        assert_eq!(
            calc_next_finished_count(Some(TodoStateEnum::Done), Some(TodoStateEnum::Done), 3),
            3
        );
        assert_eq!(
            calc_next_finished_count(Some(TodoStateEnum::Done), Some(TodoStateEnum::Wait), 3),
            3
        );
    }

    #[test]
    fn test_match_toent_filters() {
        let mut inst = ToentInst::pure_todo(
            TID::now(),
            crate::krate::toent::todoevent::TodoEvent {
                state: TodoStateEnum::Wait,
                priority: None,
            },
            0,
        );
        inst.start_tid = Some(TID::now());
        inst.end_tid = Some(TID::now());
        let req = ToentSearchReq {
            start_tid: TID::never(),
            end_tid: TID::now(),
            include_completed: false,
            include_uncompleted: true,
            include_no_time_todo: false,
            query: None,
            tags: None,
            start_index: 0,
            page_size: 20,
            chnot_otids: None,
        };
        assert!(match_toent_filters(&inst, &req));

        let mut done_inst = inst.clone();
        done_inst.todo_state = Some(TodoStateEnum::Done);
        assert!(!match_toent_filters(&done_inst, &req));

        let mut no_time_req = req.clone();
        no_time_req.include_no_time_todo = false;
        let mut no_time_inst = inst.clone();
        no_time_inst.start_tid = None;
        assert!(!match_toent_filters(&no_time_inst, &no_time_req));

        no_time_req.include_no_time_todo = true;
        assert!(match_toent_filters(&no_time_inst, &no_time_req));
    }
}
