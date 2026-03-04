use std::collections::{HashMap, HashSet};

use chin_sql::{OrderBy, SqlBuilder, Wheres, time_type::TID};
use chin_tools::AResult;
use lazy_regex::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    krate::{
        mdwt::MdwtRecord,
        toent::{
            TodoInstDto, ToentEventDefiDto, ToentEventDefiItemDto, ToentEventDto, ToentInstListRsp,
            ToentScheduleItemDto, ToentTodoDto,
            mapper::ToentReadMapper,
            po::{ToentEvent, ToentInst, ToentTodo},
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
    pub(crate) todo_closed: bool,
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

    let todo_closed = matches!(
        todo_state,
        Some(TodoStateEnum::Done | TodoStateEnum::Cancel)
    );

    ToentExtract {
        todo_priority: priority,
        todo_state,
        todo_closed,
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

impl ToentReadMapper for KDb {
    async fn toent_inst_list(
        &self,
        req: KReq<crate::krate::toent::ToentInstListReq>,
    ) -> AResult<ToentInstListRsp> {
        let start_bound = normalize_start(req.start_date.as_str());
        let end_bound = normalize_end(req.end_date.as_str());

        let conn = self.conn().await?;
        let all_filtered_insts: Vec<ToentInst> = conn
            .qry_list(
                SqlBuilder::read_all(ToentInst::TABLE)
                    .order_by([OrderBy::Asc(ToentInst::NAIVE_TIME.into())]),
                |row| -> AResult<ToentInst> { (&row).try_into() },
            )
            .await?
            .into_iter()
            .filter(|inst: &ToentInst| {
                let naive = inst.naive_time.as_str();
                naive >= start_bound.as_str() && naive <= end_bound.as_str()
            })
            .collect();

        if all_filtered_insts.is_empty() {
            return Ok(ToentInstListRsp {
                items: vec![],
                has_next: false,
                next_start: req.start_index,
            });
        }

        let page_size = req.page_size.max(1);
        let total = all_filtered_insts.len();
        let start = req.start_index.min(total);
        let end = (start + page_size).min(total);
        let has_next = end < total;
        let next_start = end;

        let insts = all_filtered_insts[start..end].to_vec();

        let mut otids: HashSet<_> = HashSet::new();
        for inst in &insts {
            otids.insert(inst.otid);
        }
        let otids: Vec<_> = otids.into_iter().collect();

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
                    todo_priority: todo.todo_priority,
                    todo_state: todo.todo_state,
                    todo_closed: todo.todo_closed,
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
                let parsed: EventDefi = serde_json::from_str(event.event_defi.as_str())
                    .unwrap_or(EventDefi { events: vec![] });
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
                    inst: TodoInstDto {
                        otid: inst_otid,
                        timezone: inst.timezone,
                        naive_time: inst.naive_time.to_string(),
                        target_status: inst.target_status,
                        note: inst.note.map(|n| n.to_string()),
                        alert_tid: inst.alert_tid,
                        target_tid: inst.target_tid,
                        tid: inst.tid,
                    },
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
