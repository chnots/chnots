use lazy_regex::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::todoevent::{TodoPriorityEnum, TodoStateEnum};

static TODO_EVENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[([A-Z]+)(?: !([A-Z]))?\]");
static STATE_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"(?i)^\s*;+\s*STATE:\s*([A-Z]+)\b");
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
    let mut state_from_bracket = None;
    let mut priority = None;
    if let Some(caps) = TODO_EVENT_REGEX.captures(content) {
        if let Some(state) = caps.get(1) {
            state_from_bracket = TodoStateEnum::try_from(state.as_str()).ok();
        }
        if let Some(pri) = caps.get(2) {
            priority = TodoPriorityEnum::try_from(pri.as_str()).ok();
        }
    }

    let mut last_state = None;
    let mut events = Vec::new();

    for line in content.lines() {
        if let Some(caps) = STATE_REGEX.captures(line)
            && let Some(state) = caps.get(1)
        {
            last_state = TodoStateEnum::try_from(state.as_str()).ok();
        }

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

    let todo_state = last_state.or(state_from_bracket);
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
