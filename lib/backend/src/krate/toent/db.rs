use std::collections::{HashMap, HashSet};

use anyhow::Context;
use chin_sql::{ILikeType, SqlBuilder, SqlReader, Wheres, time_type::TID};
use chin_tools::AResult;
use chrono::{FixedOffset, NaiveDateTime};
use lazy_regex::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    krate::{
        chnot::{ChnotKind, ChnotMeta, ChnotMetaTable},
        mdwt::db::MdwtOtidInTags,
        mdwt::{MdwtCommitReq, MdwtCommitReqData, MdwtRecord, MdwtTag, mapper::MdwtMapper},
        mdwt::{MdwtRecordTable, MdwtTagSearchType},
        toent::{
            TodoInstDto, ToentEventDefiDto, ToentEventDefiItemDto, ToentEventDto,
            ToentInstCountRsp, ToentInstListRsp, ToentScheduleItemDto, ToentSearchReq,
            ToentTodoDto, ToentTodoStateCommitReq, ToentTodoStateCommitRsp,
            mapper::ToentReadMapper,
            po::{ToentEvent, ToentTodo},
        },
    },
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier},
    model::dto::KReq,
};

use crate::krate::toent::logic::todoevent::{TodoPriorityEnum, TodoStateEnum};

static TODO_EVENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[([A-Z]+)(?: !([A-Z]))?\]");
static EVENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"(?i)^\s*;+\s*EVENT:\s*(.*?)\s*$");
static NAIVE_TIME_REGEX: Lazy<Regex> =
    lazy_regex::lazy_regex!(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})");
static TIMEZONE_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"([+-]\d{1,2}:\d{2})");

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct EventDefi {
    pub(crate) events: Vec<EventDefiItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EventDefiItem {
    pub(crate) raw: String,
    pub(crate) standard: Option<String>,
    pub(crate) timezone: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ToentExtract {
    pub(crate) todo_priority: Option<TodoPriorityEnum>,
    pub(crate) todo_state: Option<TodoStateEnum>,
    pub(crate) alert_tid: Option<TID>,
    pub(crate) start_tid: Option<TID>,
    pub(crate) end_tid: Option<TID>,
    pub(crate) timezone: Option<isize>,
    pub(crate) closed: Option<bool>,
    pub(crate) start_time: Option<TID>,
    pub(crate) start_timezone: Option<isize>,
    pub(crate) end_time: Option<TID>,
    pub(crate) end_timezone: Option<isize>,
    pub(crate) events: Vec<EventDefiItem>,
}

pub(crate) fn parse_mdwt_toent(content: &str) -> ToentExtract {
    let mut todo_state = None;
    let mut priority = None;
    if let Some(caps) = TODO_EVENT_REGEX.captures(content) {
        if let Some(state) = caps.get(1) {
            todo_state = TodoStateEnum::try_from(state.as_str()).ok();
        }
        if let Some(pri) = caps.get(2) {
            priority = TodoPriorityEnum::try_from(pri.as_str()).ok();
        }
    }

    let mut events = Vec::new();

    for line in content.lines() {
        if let Some(caps) = EVENT_REGEX.captures(line)
            && let Some(raw_match) = caps.get(1)
        {
            let raw = raw_match.as_str().trim().to_string();
            events.push(EventDefiItem {
                standard: extract_naive_time(raw.as_str()),
                timezone: extract_timezone(raw.as_str()),
                raw,
            });
        }
    }

    let closed = Some(matches!(
        todo_state,
        Some(TodoStateEnum::Done | TodoStateEnum::Cancel)
    ));
    let (start_tid, start_timezone, end_tid, end_timezone, alert_tid) =
        extract_event_time_meta(&events);

    ToentExtract {
        todo_priority: priority,
        todo_state,
        alert_tid,
        start_tid,
        end_tid,
        timezone: start_timezone,
        closed,
        start_time: start_tid,
        start_timezone,
        end_time: end_tid,
        end_timezone,
        events,
    }
}

fn extract_naive_time(input: &str) -> Option<String> {
    let caps = NAIVE_TIME_REGEX.captures(input)?;
    Some(caps.get(1)?.as_str().to_string())
}

fn extract_timezone(input: &str) -> Option<String> {
    let caps = TIMEZONE_REGEX.captures(input)?;
    let tz = caps.get(1)?.as_str();
    if tz.len() == 5 {
        Some(format!("{}0{}", &tz[..2], &tz[2..]))
    } else {
        Some(tz.to_string())
    }
}

fn extract_event_time_meta(
    events: &[EventDefiItem],
) -> (
    Option<TID>,
    Option<isize>,
    Option<TID>,
    Option<isize>,
    Option<TID>,
) {
    let mut starts = Vec::new();
    for event in events {
        let source = event.standard.as_ref().unwrap_or(&event.raw);
        let naive = extract_naive_time(source.as_str()).or_else(|| extract_naive_time(&event.raw));
        let timezone_text = extract_timezone(source.as_str())
            .or_else(|| event.timezone.clone())
            .or_else(|| extract_timezone(&event.raw));
        if let Some(naive) = naive
            && let Some(start_tid) =
                parse_tid_with_timezone(naive.as_str(), timezone_text.as_deref())
        {
            let timezone = timezone_text
                .as_deref()
                .and_then(timezone_to_offset_minutes);
            starts.push((start_tid, timezone));
        }
    }

    if starts.is_empty() {
        return (None, None, None, None, None);
    }

    starts.sort_by_key(|(tid, _)| *tid);
    let (start_tid, start_timezone) = starts[0];
    let (end_tid, end_timezone) = starts[starts.len() - 1];
    (
        Some(start_tid),
        start_timezone,
        Some(end_tid),
        end_timezone,
        None,
    )
}

fn parse_tid_with_timezone(naive: &str, timezone: Option<&str>) -> Option<TID> {
    let naive_dt = NaiveDateTime::parse_from_str(naive, "%Y-%m-%d %H:%M:%S").ok()?;
    let timestamp = if let Some(tz) = timezone {
        let fixed = normalize_timezone_text(tz)?.parse::<FixedOffset>().ok()?;
        naive_dt
            .and_local_timezone(fixed)
            .single()?
            .timestamp_millis()
    } else {
        naive_dt.and_utc().timestamp_millis()
    };
    timestamp.try_into().ok()
}

fn timezone_to_offset_minutes(timezone: &str) -> Option<isize> {
    let fixed = normalize_timezone_text(timezone)?
        .parse::<FixedOffset>()
        .ok()?;
    Some((fixed.local_minus_utc() / 60) as isize)
}

fn normalize_timezone_text(timezone: &str) -> Option<String> {
    let bytes = timezone.as_bytes();
    if bytes.len() == 5 && (bytes[0] == b'+' || bytes[0] == b'-') {
        return Some(format!(
            "{}{}:{}",
            &timezone[0..1],
            &timezone[1..3],
            &timezone[3..5]
        ));
    }
    if bytes.len() == 6 && (bytes[0] == b'+' || bytes[0] == b'-') && bytes[3] == b':' {
        return Some(timezone.to_string());
    }
    None
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

impl ToentReadMapper for KDb {
    async fn toent_inst_count(
        &self,
        req: KReq<crate::krate::toent::ToentInstCountReq>,
    ) -> AResult<ToentInstCountRsp> {
        let all_filtered_insts = self
            .load_filtered_insts(
                req.start_date.as_str(),
                req.end_date.as_str(),
                req.include_completed,
                req.include_uncompleted,
                None,
                None,
                req.get_spaces(),
            )
            .await?;

        Ok(ToentInstCountRsp {
            total: all_filtered_insts.len(),
        })
    }

    async fn toent_inst_list(
        &self,
        req: KReq<crate::krate::toent::ToentInstListReq>,
    ) -> AResult<ToentInstListRsp> {
        let all_filtered_insts = self
            .load_filtered_insts(
                req.start_date.as_str(),
                req.end_date.as_str(),
                req.include_completed,
                req.include_uncompleted,
                None,
                None,
                req.get_spaces(),
            )
            .await?;

        self.build_inst_list_rsp(all_filtered_insts, req.start_index, req.page_size)
            .await
    }

    async fn toent_search(&self, req: KReq<ToentSearchReq>) -> AResult<ToentInstListRsp> {
        let all_filtered_insts = self
            .load_filtered_insts(
                req.start_date.as_str(),
                req.end_date.as_str(),
                req.include_completed,
                req.include_uncompleted,
                req.query.clone(),
                req.tags.clone(),
                req.get_spaces(),
            )
            .await?;

        self.build_inst_list_rsp(all_filtered_insts, req.start_index, req.page_size)
            .await
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
    async fn load_filtered_insts(
        &self,
        start_date: &str,
        end_date: &str,
        include_completed: bool,
        include_uncompleted: bool,
        query: Option<String>,
        tags: Option<MdwtTagSearchType>,
        kspaces: Vec<chin_sql::str_type::Varchar<40>>,
    ) -> AResult<Vec<TodoInstDto>> {
        if !include_completed && !include_uncompleted {
            return Ok(vec![]);
        }

        let start_bound = normalize_start(start_date);
        let end_bound = normalize_end(end_date);
        let searchable_otids = self
            .load_searchable_mdwt_otids(query, tags, kspaces)
            .await?;
        if searchable_otids.is_empty() {
            return Ok(vec![]);
        }

        let otids: Vec<_> = searchable_otids.iter().copied().collect();

        let conn = self.conn().await?;
        let todos: HashMap<_, _> = conn
            .qry_list(
                SqlBuilder::read_all(ToentTodo::TABLE)
                    .r#where(Wheres::r#in(ToentTodo::OTID, otids.clone())),
                |row| (&row).try_into(),
            )
            .await?
            .into_iter()
            .map(|todo: ToentTodo| (todo.otid, todo))
            .collect();

        let mut all_filtered_insts: Vec<TodoInstDto> = Vec::new();
        let events: Vec<ToentEvent> = conn
            .qry_list(
                SqlBuilder::read_all(ToentEvent::TABLE)
                    .r#where(Wheres::r#in(ToentEvent::OTID, otids)),
                |row| (&row).try_into(),
            )
            .await?;

        for event in events {
            let todo = todos.get(&event.otid);
            let target_status = todo.and_then(|t| t.todo_state);
            let note = todo.and_then(|t| t.note.as_ref().map(|n| n.to_string()));
            let alert_tid = todo.and_then(|t| t.alert_tid);
            let parsed: EventDefi =
                serde_json::from_str(event.event_defi.as_str()).unwrap_or_default();

            for item in parsed.events {
                let Some(target) = EventDefi::resolve_to_target(item) else {
                    continue;
                };
                let inst = TodoInstDto {
                    otid: event.otid,
                    timezone: target.timezone,
                    naive_time: target.naive_time,
                    target_status,
                    note: note.clone(),
                    alert_tid,
                    target_tid: target.utc.timestamp_millis().try_into()?,
                    tid: event.tid,
                };
                if inst_match(
                    &inst,
                    start_bound.as_str(),
                    end_bound.as_str(),
                    include_completed,
                    include_uncompleted,
                ) {
                    all_filtered_insts.push(inst);
                }
            }
        }

        all_filtered_insts.sort_by(|a, b| a.naive_time.cmp(&b.naive_time));

        Ok(all_filtered_insts)
    }

    async fn load_searchable_mdwt_otids(
        &self,
        query: Option<String>,
        tags: Option<MdwtTagSearchType>,
        kspaces: Vec<chin_sql::str_type::Varchar<40>>,
    ) -> AResult<HashSet<TID>> {
        let conn = self.conn().await?;
        let cm = ChnotMetaTable::new("cm");

        let base_otids: HashSet<TID> = conn
            .qry_list(
                SqlReader::read(cm.otid(), &cm)
                    .wheres(Wheres::and([
                        cm.kspace().v_in(kspaces.clone()),
                        cm.kind().v_eq(ChnotKind::MarkdownWithToent),
                    ]))
                    .build2(),
                |row| -> AResult<TID> { row.try_get(ChnotMeta::OTID) },
            )
            .await?
            .into_iter()
            .collect();

        if base_otids.is_empty() {
            return Ok(base_otids);
        }

        let mut result_otids = base_otids;

        if let Some(tag_constraint) =
            MdwtOtidInTags::new("toent_tag_constraint").sub_query_table(tags, kspaces)
        {
            let tag_otids: HashSet<TID> = conn
                .qry_list(tag_constraint, |row| -> AResult<TID> {
                    row.try_get(MdwtTag::MDWT_OTID)
                })
                .await?
                .into_iter()
                .collect();
            result_otids.retain(|otid| tag_otids.contains(otid));
        }

        if let Some(query) = query.as_ref().map(|v| v.trim()).filter(|v| !v.is_empty()) {
            let mr = MdwtRecordTable::new("mr");
            let query_otids: HashSet<TID> = conn
                .qry_list(
                    SqlReader::read(mr.otid(), &mr)
                        .wheres(Wheres::and([
                            mr.otid()
                                .v_in(result_otids.iter().copied().collect::<Vec<_>>()),
                            mr.content().v_ilike(query, ILikeType::Fuzzy),
                        ]))
                        .build2(),
                    |row| -> AResult<TID> { row.try_get(MdwtRecord::OTID) },
                )
                .await?
                .into_iter()
                .collect();
            result_otids.retain(|otid| query_otids.contains(otid));
        }

        Ok(result_otids)
    }

    async fn build_inst_list_rsp(
        &self,
        all_filtered_insts: Vec<TodoInstDto>,
        start_index: usize,
        page_size: usize,
    ) -> AResult<ToentInstListRsp> {
        if all_filtered_insts.is_empty() {
            return Ok(ToentInstListRsp {
                items: vec![],
                has_next: false,
                next_start: start_index,
            });
        }

        let page_size = page_size.max(1);
        let total = all_filtered_insts.len();
        let start = start_index.min(total);
        let end = (start + page_size).min(total);
        let has_next = end < total;
        let next_start = end;

        let insts = all_filtered_insts[start..end].to_vec();

        let mut otids: HashSet<_> = HashSet::new();
        for inst in &insts {
            otids.insert(inst.otid);
        }
        let otids: Vec<_> = otids.into_iter().collect();

        let conn = self.conn().await?;
        let todos: HashMap<_, _> = conn
            .qry_list(
                SqlBuilder::read_all(ToentTodo::TABLE)
                    .r#where(Wheres::r#in(ToentTodo::OTID, otids.clone())),
                |row| (&row).try_into(),
            )
            .await?
            .into_iter()
            .map(|todo: ToentTodo| {
                let otid = todo.otid;
                let dto = ToentTodoDto {
                    otid,
                    todo_state: todo.todo_state,
                    todo_priority: todo.todo_priority,
                    alert_tid: todo.alert_tid,
                    start_tid: todo.start_tid,
                    end_tid: todo.end_tid,
                    timezone: todo.timezone,
                    closed: todo.closed,
                    tid: todo.tid,
                };
                (otid, dto)
            })
            .collect();

        let events: HashMap<_, _> = conn
            .qry_list(
                SqlBuilder::read_all(ToentEvent::TABLE)
                    .r#where(Wheres::r#in(ToentEvent::OTID, otids.clone())),
                |row| (&row).try_into(),
            )
            .await?
            .into_iter()
            .map(|event: ToentEvent| {
                let otid = event.otid;
                let parsed: EventDefi =
                    serde_json::from_str(event.event_defi.as_str()).unwrap_or_default();
                let dto = ToentEventDto {
                    otid,
                    event_defi: ToentEventDefiDto {
                        events: parsed
                            .events
                            .into_iter()
                            .map(|item| ToentEventDefiItemDto {
                                raw: item.raw,
                                standard: item.standard,
                                timezone: item.timezone,
                            })
                            .collect(),
                    },
                    start_time: event.start_time,
                    start_timezone: event.start_timezone,
                    end_time: event.end_time,
                    end_timezone: event.end_timezone,
                    tid: event.tid,
                };
                (otid, dto)
            })
            .collect();

        let titles: HashMap<_, _> = conn
            .qry_list(
                SqlBuilder::read_all(MdwtRecord::TABLE)
                    .r#where(Wheres::r#in(MdwtRecord::OTID, otids)),
                |row| -> AResult<(TID, String)> {
                    Ok((
                        row.try_get(MdwtRecord::OTID)?,
                        row.try_get(MdwtRecord::CONTENT)?,
                    ))
                },
            )
            .await?
            .into_iter()
            .map(|(otid, content)| (otid, first_non_empty_line(content.as_str())))
            .collect();

        let items = insts
            .into_iter()
            .map(|inst| {
                let inst_otid = inst.otid;
                ToentScheduleItemDto {
                    inst,
                    todo: todos.get(&inst_otid).cloned(),
                    event: events.get(&inst_otid).cloned(),
                    title: titles.get(&inst_otid).cloned(),
                }
            })
            .collect();

        Ok(ToentInstListRsp {
            items,
            has_next,
            next_start,
        })
    }
}

fn is_done(state: Option<TodoStateEnum>) -> bool {
    matches!(state, Some(TodoStateEnum::Done))
}

fn inst_match(
    inst: &TodoInstDto,
    start_bound: &str,
    end_bound: &str,
    include_completed: bool,
    include_uncompleted: bool,
) -> bool {
    let naive = inst.naive_time.as_str();
    if naive < start_bound || naive > end_bound {
        return false;
    }

    let done = is_done(inst.target_status);
    (done && include_completed) || (!done && include_uncompleted)
}

fn normalize_start(input: &str) -> String {
    if input.len() == 10 {
        format!("{input} 00:00:00")
    } else {
        input.to_string()
    }
}

fn normalize_end(input: &str) -> String {
    if input.len() == 10 {
        format!("{input} 23:59:59")
    } else {
        input.to_string()
    }
}

fn first_non_empty_line(content: &str) -> String {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .unwrap_or_default()
}
