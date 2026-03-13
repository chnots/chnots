use std::collections::{HashMap, HashSet};

use anyhow::{Context, bail};
use chin_sql::{
    Froms, ILikeType, JoinTable, Joins, LimitOffset, SqlBuilder, SqlReader, SqlTable, Wheres,
    str_type::Text, time_type::TID,
};
use chin_tools::{AResult, EResult};
use chrono::Utc;
use lazy_regex::Lazy;
use regex::Regex;

use crate::{
    krate::{
        chnot::ChnotMetaTable,
        mdwt::{
            MdwtCommitReq, MdwtCommitReqData, MdwtRecord, MdwtRecordTable, db::MdwtOtidInTags,
            mapper::MdwtMapper,
        },
        toent::{
            ToentInstCountReq, ToentInstCountRsp, ToentSearchReq, ToentSearchRsp,
            ToentSearchRspData, ToentTodoStateCommitReq, ToentTodoStateCommitRsp,
            logic::timeevent::{
                TimeEvent,
                repeater::{RepeatType, endconditon::EndCondition, interval::TimeInterval},
                timeenum::{TimeEnum, UtcWithOffsetType},
            },
            mapper::ToentMapper,
            po::{
                ToentDefi, ToentDefiTable, ToentEventDefi, ToentInst, ToentInstTable,
                toent_defi_by_todo,
            },
            todoevent::TodoEvent,
        },
    },
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier, KDbTx},
    model::{KSerde, dto::KReq},
};

use crate::krate::toent::logic::todoevent::TodoStateEnum;

static TODO_EVENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[([A-Z]+)(?: !([A-Z]))?\]");
const MICROS_PER_SECOND: i64 = 1_000_000;
const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

fn current_tid() -> Option<TID> {
    Utc::now().timestamp_micros().try_into().ok()
}

fn time_interval_step_micros(interval: &TimeInterval) -> Option<i64> {
    let years = interval.date.year.unwrap_or(0);
    let months = interval.date.month.unwrap_or(0);
    if years != 0 || months != 0 {
        return None;
    }

    let total_seconds = i64::from(interval.date.day.unwrap_or(0)) * SECONDS_PER_DAY
        + i64::from(interval.week.unwrap_or(0)) * 7 * SECONDS_PER_DAY
        + i64::from(interval.time.hour.unwrap_or(0)) * 3600
        + i64::from(interval.time.minute.unwrap_or(0)) * 60
        + i64::from(interval.time.second.unwrap_or(0));

    (total_seconds > 0).then_some(total_seconds * MICROS_PER_SECOND)
}

fn rewrite_todo_state(content: &str, todo_state: TodoStateEnum) -> AResult<String> {
    let caps = TODO_EVENT_REGEX
        .captures(content)
        .context("unable to find todo marker from mdwt content")?;
    let priority = caps
        .get(2)
        .map(|p| format!(" !{}", p.as_str()))
        .unwrap_or_default();
    let replacement = format!("[{}{}]", todo_state.as_static_str(), priority);
    Ok(TODO_EVENT_REGEX
        .replacen(content, 1, replacement.as_str())
        .to_string())
}

fn repeat_todo_step_micros(defi: &ToentEventDefi) -> Option<i64> {
    defi.data.iter().find_map(|item| {
        let (interval, repeat_type) = item.interval?;
        if repeat_type != RepeatType::RepeatTodo {
            return None;
        }
        time_interval_step_micros(&interval)
    })
}

#[derive(Clone)]
struct RepeatPlan {
    repeat_type: RepeatType,
    step_micros: i64,
    start_tid: TID,
    total_count: Option<u32>,
}

fn first_event_start_tid(event: &TimeEvent) -> Option<TID> {
    let start = event.start?;
    let ts = start.to_utc_timestamp().ok()?;
    match ts {
        UtcWithOffsetType::Point(point) => Some(point.utc()),
        UtcWithOffsetType::Period { start, .. } => Some(start.utc()),
    }
}

fn event_total_count(event: &TimeEvent) -> Option<u32> {
    fn extract(cond: Option<EndCondition>) -> Option<u32> {
        match cond {
            Some(EndCondition::Times(times)) => Some(times.count()),
            _ => None,
        }
    }

    extract(event.end).or_else(|| extract(event.interval_end))
}

fn build_repeat_plan(defi: &ToentEventDefi) -> Option<RepeatPlan> {
    defi.data.iter().find_map(|event| {
        let (interval, repeat_type) = event.interval?;
        if !matches!(
            repeat_type,
            RepeatType::RepeatEvent | RepeatType::RepeatTodo
        ) {
            return None;
        }
        let step_micros = time_interval_step_micros(&interval)?;
        let start_tid = first_event_start_tid(event)?;

        Some(RepeatPlan {
            repeat_type,
            step_micros,
            start_tid,
            total_count: event_total_count(event),
        })
    })
}

fn has_lunar_timeevent(defi: &ToentEventDefi) -> bool {
    defi.data
        .iter()
        .any(|event| matches!(event.start, Some(TimeEnum::Chn(_))))
}

fn in_search_window(inst: &ToentInst, req: &ToentSearchReq) -> bool {
    let in_time = match (inst.start_tid, inst.end_tid) {
        (Some(start), Some(end)) => {
            end.as_num() > req.start_tid.as_num() && start.as_num() < req.end_tid.as_num()
        }
        (Some(start), None) => start.as_num() < req.end_tid.as_num(),
        (None, Some(end)) => end.as_num() > req.start_tid.as_num(),
        (None, None) => req.include_no_time_todo,
    };

    let done = is_done(inst.todo_state);
    in_time && ((done && req.include_completed) || (!done && req.include_uncompleted))
}

fn generated_inst(
    template: &ToentInst,
    start_tid: TID,
    repeat_type: RepeatType,
    seq: i64,
) -> ToentInst {
    let mut next = template.clone();
    if let Some(base_start) = template.start_tid {
        if let (Some(base_end), Some(delta)) = (
            template.end_tid,
            template.end_tid.and_then(|e| e.checked_sub(*base_start)),
        ) {
            let _ = base_end;
            next.end_tid = start_tid.add_micros(delta);
        }
    }
    next.start_tid = Some(start_tid);
    if repeat_type == RepeatType::RepeatTodo {
        next.todo_state = Some(TodoStateEnum::Todo);
    }
    next.tid = start_tid.add_micros(seq).unwrap_or_default();
    next
}

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
            };
            let count = self.count_search_total(req.frame(count_req)).await?;
            totals.insert(range.key.clone(), count);
        }

        Ok(ToentInstCountRsp { total, totals })
    }

    async fn toent_search(&self, req: KReq<ToentSearchReq>) -> AResult<ToentSearchRsp> {
        if !req.include_completed && !req.include_uncompleted {
            return Ok(ToentSearchRsp {
                items: vec![],
                has_next: false,
                next_start: req.start_index,
            });
        }

        let tags_req = req.tags.clone();
        let mkspaces = req.get_spaces();

        let tag_otids = MdwtOtidInTags::new("tag_otids");
        let tags = tag_otids.sub_query_table(tags_req);
        let mr = MdwtRecordTable::new("mr");
        let cm = ChnotMetaTable::new("cm");
        let td = ToentDefiTable::new("td");
        struct EventDefiAndContent {
            content: Text,
            toent_defi: ToentDefi,
        }

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
                    Wheres::and([
                        td.end_tid().v_cmp(">", req.start_tid),
                        td.start_tid().v_cmp("<", req.end_tid),
                    ]),
                    Wheres::if_some(req.include_no_time_todo.then_some(()), |_| {
                        Wheres::and([
                            td.todo_flag().v_eq(true),
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
                cm.kspace().v_in(mkspaces),
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
        let ti_sql = SqlReader::read(ti.all_fields(), &ti)
            .wheres(ti.otid().v_in(otids))
            .build2();

        let insts: Vec<ToentInst> = self
            .conn()
            .await?
            .qry_list(ti_sql, |row| ToentInst::try_from_kdb_row(&row))
            .await?;

        let dm: HashMap<TID, EventDefiAndContent> =
            defis.into_iter().map(|d| (d.toent_defi.otid, d)).collect();
        let mut inst_group: HashMap<TID, Vec<ToentInst>> = HashMap::new();
        for inst in insts {
            inst_group.entry(inst.otid).or_default().push(inst);
        }

        let mut items: Vec<ToentSearchRspData> = vec![];
        for (otid, md) in dm {
            let mut current = inst_group.remove(&otid).unwrap_or_default();
            let title = first_non_empty_line(md.content.as_str());

            let start_set: HashSet<TID> =
                current.iter().filter_map(|inst| inst.start_tid).collect();

            let mut generated: Vec<ToentInst> = vec![];
            if let Some(defi) = &md.toent_defi.event_defi
                && let Some(plan) = build_repeat_plan(defi)
            {
                let event_end_tid = defi.end_time().map(|end| end.utc());
                let limit_tid = event_end_tid
                    .map(|end| end.min(req.end_tid))
                    .unwrap_or(req.end_tid);

                let latest_inst = current.iter().cloned().max_by(|left, right| {
                    let left_key = left.start_tid.unwrap_or_default().as_num();
                    let right_key = right.start_tid.unwrap_or_default().as_num();
                    left_key
                        .cmp(&right_key)
                        .then_with(|| left.tid.as_num().cmp(&right.tid.as_num()))
                });

                let base = match plan.repeat_type {
                    RepeatType::RepeatEvent => Some(plan.start_tid),
                    RepeatType::RepeatTodo => latest_inst
                        .as_ref()
                        .and_then(|inst| inst.start_tid)
                        .or(Some(plan.start_tid)),
                    RepeatType::Once => None,
                };

                let template = latest_inst.unwrap_or(ToentInst {
                    otid,
                    todo_state: Some(TodoStateEnum::Todo),
                    todo_priority: None,
                    alert_tid: None,
                    start_tid: Some(plan.start_tid),
                    end_tid: Some(plan.start_tid),
                    finished_count: 0,
                    timezone: None,
                    closed: None,
                    tid: TID::default(),
                    note: None,
                    is_lunar: false,
                });

                let mut cursor = base;
                let mut seq: i64 = 1;
                let repeat_budget = if plan.repeat_type == RepeatType::RepeatTodo {
                    plan.total_count.map(|total| {
                        let finished = template.finished_count.max(0) as u32;
                        total.saturating_sub(finished.saturating_add(1))
                    })
                } else {
                    plan.total_count
                };
                let mut remain = repeat_budget;

                while let Some(curr_start) = cursor {
                    let Some(next_start) = curr_start.add_micros(plan.step_micros) else {
                        break;
                    };
                    if next_start.as_num() > limit_tid.as_num() {
                        break;
                    }

                    if remain.is_some_and(|count| count == 0) {
                        break;
                    }

                    if !start_set.contains(&next_start) {
                        let next = generated_inst(&template, next_start, plan.repeat_type, seq);
                        generated.push(next);
                    }

                    if let Some(count) = remain.as_mut()
                        && *count > 0
                    {
                        *count -= 1;
                    }

                    cursor = Some(next_start);
                    seq += 1;
                    if seq > 10_000 {
                        break;
                    }
                }
            }

            current.extend(generated);
            for inst in current
                .into_iter()
                .filter(|inst| in_search_window(inst, &req))
            {
                items.push(ToentSearchRspData {
                    inst,
                    defi: md.toent_defi.event_defi.clone(),
                    title: title.clone(),
                });
            }
        }

        items.sort_by(|left, right| {
            let ls = left.inst.start_tid.unwrap_or_default().as_num();
            let rs = right.inst.start_tid.unwrap_or_default().as_num();
            ls.cmp(&rs)
                .then_with(|| left.inst.tid.as_num().cmp(&right.inst.tid.as_num()))
        });

        Ok(ToentSearchRsp {
            next_start,
            items,
            has_next,
        })
    }

    async fn toent_todo_state_commit(
        &self,
        req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp> {
        let content: String = self
            .conn()
            .await?
            .qry_opt(
                SqlBuilder::read(MdwtRecord::TABLE, &[MdwtRecord::CONTENT])
                    .r#where(Wheres::equal(MdwtRecord::OTID, req.otid)),
                |row| -> AResult<String> { row.try_get(MdwtRecord::CONTENT) },
            )
            .await?
            .context(format!("unable to find mdwt by otid {}", req.otid))?;

        let next_content = rewrite_todo_state(content.as_str(), req.todo_state)?;

        self.mdwt_commit(req.frame(MdwtCommitReq {
            mdwt: MdwtCommitReqData {
                otid: req.otid,
                content: next_content.into(),
            },
        }))
        .await?;

        Ok(ToentTodoStateCommitRsp {
            otid: req.otid,
            todo_state: req.todo_state,
        })
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
            };

            let rsp = self.toent_search(req.frame(search_req)).await?;
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
    /// 首先判断之前 toent_inst 中是否存在实例。
    /// - 如果不存在，直接写入新的，并且将 finished_count 标记为0
    /// - 如果存在
    ///   - 如果新的状态和之前的状态一样，不写入新数据
    ///   - 如果新状态为 done，先写入一条新的 done 数据,且 finished_count + 1
    ///     - 如果需要重复，并且还未结束
    ///       - 写入一条新的 todo 数据
    ///
    pub(crate) async fn toent_commit(
        &self,
        otid: TID,
        todo_event: &Option<TodoEvent>,
        time_event_defi: &Option<ToentEventDefi>,
        start_tid: Option<TID>,
    ) -> EResult {
        let td = ToentDefiTable::new("td");
        let saved_defi: Option<ToentDefi> = self
            .qry_opt(
                SqlReader::read(td.all_fields(), &td)
                    .wheres(td.otid().v_eq(otid))
                    .build2(),
                |row| ToentDefi::try_from_kdb_row(&row),
            )
            .await?;

        let next_event_po = match time_event_defi {
            Some(defi) if !defi.data.is_empty() => Some(defi.to_po(otid, todo_event.is_some())),
            _ => todo_event
                .as_ref()
                .and_then(|_| toent_defi_by_todo(otid, true)),
        };
        match (&saved_defi, &next_event_po) {
            (Some(saved), Some(next))
                if saved.event_defi != next.event_defi || saved.todo_flag != next.todo_flag =>
            {
                self.po_otid_insert([next.clone()]).await?;
            }
            (None, Some(next)) => {
                self.po_otid_insert([next.clone()]).await?;
            }
            (Some(_), None) => {
                self.omit_rows::<ToentDefi>(Wheres::equal(ToentDefi::OTID, otid))
                    .await?;
            }
            _ => {}
        }

        let ti = ToentInstTable::new("ti");
        let saved_inst: Option<ToentInst> = self
            .qry_list(
                SqlReader::read(ti.all_fields(), &ti)
                    .wheres(ti.otid().v_eq(otid))
                    .build2(),
                |row| ToentInst::try_from_kdb_row(&row),
            )
            .await?
            .into_iter()
            .max_by(|left, right| {
                let left_start = left.start_tid.unwrap_or_default().as_num();
                let right_start = right.start_tid.unwrap_or_default().as_num();
                left_start
                    .cmp(&right_start)
                    .then_with(|| left.tid.as_num().cmp(&right.tid.as_num()))
            });

        let prev_state = saved_inst.as_ref().and_then(|inst| inst.todo_state);
        let mut finished_count = saved_inst
            .as_ref()
            .map(|inst| inst.finished_count)
            .unwrap_or(0);
        let repeat_todo_step = time_event_defi
            .as_ref()
            .and_then(|v| repeat_todo_step_micros(v));
        if let Some(todo) = todo_event {
            if saved_inst.is_some()
                && prev_state != Some(todo.state)
                && todo.state == TodoStateEnum::Done
            {
                finished_count += 1;
            }
        }

        let (event_start_tid, event_end_tid, event_timezone) = match next_event_po.as_ref() {
            Some(po) => (
                (po.start_tid.unwrap_or_default() != TID::never())
                    .then_some(po.start_tid)
                    .unwrap_or_default(),
                (po.end_tid.unwrap_or_default() != TID::never())
                    .then_some(po.end_tid)
                    .unwrap_or_default(),
                po.start_timezone,
            ),
            None => (None, None, None),
        };

        let mut next_state = todo_event.as_ref().map(|todo| todo.state);
        let mut next_start_tid = start_tid.or(event_start_tid);
        let next_is_lunar = next_event_po
            .as_ref()
            .and_then(|po| po.event_defi.as_ref())
            .is_some_and(has_lunar_timeevent);
        if matches!(next_state, Some(TodoStateEnum::Done)) {
            if let (Some(step), Some(now_tid)) = (repeat_todo_step, current_tid()) {
                if let Some(next_num) = now_tid.as_num().checked_add(step) {
                    if let Ok(next_tid) = TID::try_from(next_num) {
                        next_start_tid = Some(next_tid);
                        next_state = Some(TodoStateEnum::Todo);
                    }
                }
            }
        }

        let next_inst = ToentInst {
            otid,
            todo_state: next_state,
            todo_priority: todo_event.as_ref().and_then(|todo| todo.priority),
            alert_tid: saved_inst.as_ref().and_then(|inst| inst.alert_tid),
            start_tid: next_start_tid,
            end_tid: event_end_tid,
            timezone: event_timezone,
            closed: saved_inst.as_ref().and_then(|inst| inst.closed),
            tid: TID::default(),
            note: saved_inst.as_ref().and_then(|inst| inst.note.clone()),
            finished_count,
            is_lunar: next_is_lunar,
        };

        if let Some(saved) = &saved_inst {
            if saved.todo_state == next_inst.todo_state
                && saved.todo_priority == next_inst.todo_priority
                && saved.start_tid == next_inst.start_tid
                && saved.end_tid == next_inst.end_tid
                && saved.timezone == next_inst.timezone
                && saved.finished_count == next_inst.finished_count
            {
                return Ok(());
            }
        }

        if next_inst.todo_state.is_some()
            || next_inst.start_tid.is_some()
            || next_inst.end_tid.is_some()
        {
            self.po_otid_insert([next_inst]).await?;
        }

        Ok(())
    }
}

fn is_done(state: Option<TodoStateEnum>) -> bool {
    matches!(state, Some(TodoStateEnum::Done))
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
        mapper::db::HistCreateSql,
    };

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
}
