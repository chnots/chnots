use std::collections::{HashMap, HashSet};

use chin_sql::{GenerateTableSchema, str_type::Text, time_type::TID};
use chin_tools::AResult;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    impl_otid_support,
    krate::toent::{
        logic::todoevent::{TodoPriorityEnum, TodoStateEnum},
        timeevent::{
            TimeEvent, TimeEventInst,
            repeater::interval::TimeInterval,
            timeenum::{
                TimeEnum, Timestamp, UtcWithOffset, UtcWithOffsetType, chinese::ChnTimeCalculator,
                westen::WesTime,
            },
        },
        todoevent::TodoEvent,
    },
    mapper::{
        Curd,
        db::{KDbRow, KDbRowBehavier},
    },
};

fn todo_state_enum_to_sql(this: Option<TodoStateEnum>) -> Option<String> {
    this.map(|e| e.as_static_str().into())
}

fn todo_priority_enum_to_sql(this: Option<TodoPriorityEnum>) -> Option<i64> {
    this.map(|e| e.as_priority().into())
}

fn timezone_to_sql(this: Option<isize>) -> Option<i64> {
    this.map(|e| e as i64)
}

fn timeevent_to_sql(this: Option<TimeEventField>) -> Option<Text> {
    match this {
        Some(this) => serde_json::to_string(&this).ok().map(|e| e.into()),
        None => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, PartialEq)]
pub(crate) struct ToentInst {
    // start_tid, because part of primary must not by null.
    #[gts_type = "i64"]
    #[gts_primary]
    pub otid: TID,

    #[gts_key]
    #[gts_type = "i64"]
    pub chnot_otid: TID,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    #[gts_type = "i64"]
    pub alert_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_key]
    pub start_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_key]
    pub end_tid: Option<TID>,

    pub finished_count: i64,
    pub is_lunar: bool,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub timezone: Option<isize>,

    pub closed: Option<bool>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub note: Option<Text>,
}

impl ToentInst {
    pub(crate) fn pure_todo(chnot_otid: TID, todo_event: TodoEvent, finished_count: i64) -> Self {
        ToentInst {
            chnot_otid,
            todo_state: Some(todo_event.state),
            todo_priority: todo_event.priority,
            alert_tid: None,
            start_tid: None,
            end_tid: None,
            timezone: None,
            closed: None,
            tid: TID::now(),
            note: None,
            finished_count,
            is_lunar: false,
            otid: chnot_otid,
        }
    }
}

impl ToentDefi {
    pub(crate) fn from_todo(
        otid: TID,
        todo_state: TodoStateEnum,
        todo_priority: Option<TodoPriorityEnum>,
    ) -> Option<ToentDefi> {
        Some(ToentDefi {
            otid: otid,
            event_defi: None,
            todo_state: Some(todo_state),
            start_tid: None,
            start_timezone: None,
            end_tid: None,
            end_timezone: None,
            total_count: None,
            tid: TID::now(),
            todo_priority,
        })
    }
}

impl TryFrom<&KDbRow> for ToentInst {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let timezone: Option<i64> = value.try_get(Self::TIMEZONE)?;
        Ok(Self {
            chnot_otid: value.try_get(Self::CHNOT_OTID)?,
            tid: value.try_get(Self::TID)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
            alert_tid: value.try_get(Self::ALERT_TID)?,
            start_tid: value.try_get(Self::START_TID)?,
            end_tid: value.try_get(Self::END_TID)?,
            timezone: timezone.map(|v| v as isize),
            closed: value.try_get(Self::CLOSED)?,
            note: value.try_get(Self::NOTE)?,
            finished_count: value.try_get(Self::FINISHED_COUNT)?,
            is_lunar: value.try_get(Self::IS_LUNAR)?,
            otid: value.try_get(Self::OTID)?,
        })
    }
}

impl Curd for ToentInst {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ToentInst}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentDefi {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Text"]
    #[gts_tosql = "timeevent_to_sql"]
    pub event_defi: Option<TimeEventField>,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    #[gts_type = "i64"]
    #[gts_key]
    pub start_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub start_timezone: Option<isize>,

    #[gts_type = "i64"]
    #[gts_key]
    pub end_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub end_timezone: Option<isize>,

    total_count: Option<i64>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct TimeEventField {
    pub time_events: HashSet<TimeEvent>,
}

impl TimeEventField {
    pub(crate) fn start_time(&self) -> Option<UtcWithOffset> {
        self.time_events
            .iter()
            .filter_map(|event| {
                let start = event.start?;
                let ts = start.to_utc_timestamp().ok()?;
                match ts {
                    crate::krate::toent::timeevent::timeenum::UtcWithOffsetType::Point(point) => {
                        Some(point)
                    }
                    crate::krate::toent::timeevent::timeenum::UtcWithOffsetType::Period {
                        start,
                        ..
                    } => Some(start),
                }
            })
            .min()
    }

    pub(crate) fn end_time(&self) -> Option<UtcWithOffset> {
        self.time_events
            .iter()
            .filter_map(|event| match event.end {
                Some(end_cond) => match end_cond {
                    crate::krate::toent::timeevent::repeater::endconditon::EndCondition::Times(_) => {
                        None
                    }
                    crate::krate::toent::timeevent::repeater::endconditon::EndCondition::Interval(
                        interval,
                    ) => calc_interval_end_time(event.start, interval),
                    crate::krate::toent::timeevent::repeater::endconditon::EndCondition::Time(
                        time_enum,
                    ) => {
                        let ts = time_enum.to_utc_timestamp().ok()?;
                        match ts {
                            crate::krate::toent::timeevent::timeenum::UtcWithOffsetType::Point(
                                point,
                            ) => Some(point),
                            crate::krate::toent::timeevent::timeenum::UtcWithOffsetType::Period {
                                end,
                                ..
                            } => Some(end),
                        }
                    }
                },
                None => None,
            })
            .max()
    }

    pub(crate) fn to_po(
        &self,
        otid: TID,
        todo_state: Option<TodoStateEnum>,
        todo_priority: Option<TodoPriorityEnum>,
    ) -> ToentDefi {
        let start = self.start_time();
        let end = self.end_time().or(start);

        let (start_tid, start_timezone) = start
            .map(|v| (v.utc(), Some(v.local_minus_utc() as isize)))
            .unwrap_or((TID::never(), None));

        let (end_tid, end_timezone) = end
            .map(|v| (v.utc(), Some(v.local_minus_utc() as isize)))
            .unwrap_or((TID::never(), None));

        ToentDefi {
            otid,
            event_defi: Some(self.clone()),
            start_tid: Some(start_tid),
            start_timezone,
            end_tid: Some(end_tid),
            end_timezone,
            total_count: None,
            tid: TID::now(),
            todo_state,
            todo_priority,
        }
    }

    pub(crate) fn generate(
        &self,
        otid: TID,
        todo_flag: bool,
        todo_priority: Option<TodoPriorityEnum>,
        max_count: usize,
        last_finish_uwo: Option<UtcWithOffset>,
        window_start_uwo: Option<UtcWithOffset>,
        window_end_uwo: Option<UtcWithOffset>,
    ) -> AResult<Vec<ToentInst>> {
        let generated: Vec<TimeEventInst> = self
            .time_events
            .iter()
            .map(|ti| ti.generate(max_count, last_finish_uwo, window_start_uwo, window_end_uwo))
            .collect::<AResult<Vec<_>>>()?
            .into_iter()
            .flat_map(|ti| ti)
            .sorted_by(|e1, e2| e1.start_tid.cmp(&e2.start_tid))
            .collect();
        let todo_state = if todo_flag {
            Some(TodoStateEnum::Wait)
        } else {
            None
        };
        let mut hists = HashMap::new();
        for (id, inst) in generated.into_iter().enumerate() {
            if id < max_count {
                hists.entry(inst.start_tid).or_insert(ToentInst {
                    chnot_otid: otid,
                    todo_state,
                    todo_priority,
                    alert_tid: inst.alert_tid,
                    start_tid: inst.start_tid,
                    otid: inst.start_tid.unwrap_or_default(),
                    end_tid: inst.end_tid,
                    finished_count: 0,
                    is_lunar: inst.is_lunar,
                    timezone: inst.timezone,
                    closed: None,
                    tid: TID::now(),
                    note: None,
                });
            }
        }
        Ok(hists.values().map(|e| e.clone()).collect())
    }

    pub fn has_lunar_timeevent(&self) -> bool {
        self.time_events
            .iter()
            .any(|event| matches!(event.start, Some(TimeEnum::Chn(_))))
    }
}

fn first_utc(ts: UtcWithOffsetType) -> UtcWithOffset {
    match ts {
        UtcWithOffsetType::Point(point) => point,
        UtcWithOffsetType::Period { start, .. } => start,
    }
}

fn last_utc(ts: UtcWithOffsetType) -> UtcWithOffset {
    match ts {
        UtcWithOffsetType::Point(point) => point,
        UtcWithOffsetType::Period { end, .. } => end,
    }
}

fn calc_interval_end_time(
    start: Option<crate::krate::toent::timeevent::timeenum::TimeEnum>,
    interval: TimeInterval,
) -> Option<UtcWithOffset> {
    let start = start?;

    match start {
        crate::krate::toent::timeevent::timeenum::TimeEnum::Chn(chn) => {
            let end = ChnTimeCalculator::add(&chn, interval).ok()?;
            Some(last_utc(end.to_utc_timestamp().ok()?))
        }
        crate::krate::toent::timeevent::timeenum::TimeEnum::Wes(wes) => {
            calc_wes_interval_end_time(&wes, interval)
        }
    }
}

fn calc_wes_interval_end_time(wes: &WesTime, interval: TimeInterval) -> Option<UtcWithOffset> {
    let start = first_utc(wes.to_utc_timestamp().ok()?);
    let offset = start.local_minus_utc();
    let utc_dt = start.utc().as_utc().naive_utc();
    let local_dt = utc_dt + Duration::seconds(i64::from(offset));

    let with_ym = add_gregorian_year_month(
        local_dt,
        interval.date.year.unwrap_or(0),
        interval.date.month.unwrap_or(0),
    )?;

    let day_offset =
        i64::from(interval.date.day.unwrap_or(0)) + i64::from(interval.week.unwrap_or(0)) * 7;
    let end_local = with_ym
        + Duration::days(day_offset)
        + Duration::hours(i64::from(interval.time.hour.unwrap_or(0)))
        + Duration::minutes(i64::from(interval.time.minute.unwrap_or(0)))
        + Duration::seconds(i64::from(interval.time.second.unwrap_or(0)));

    let end_utc_seconds = end_local.and_utc().timestamp() - i64::from(offset);
    let end_utc = TID::try_from(end_utc_seconds * 1_000_000).ok()?;

    Some(UtcWithOffset::new(end_utc, offset))
}

fn add_gregorian_year_month(base: NaiveDateTime, years: i32, months: i32) -> Option<NaiveDateTime> {
    let total_month = base.year() * 12 + (base.month() as i32 - 1) + years * 12 + months;
    let target_year = total_month.div_euclid(12);
    let target_month = total_month.rem_euclid(12) + 1;

    let mut day = base.day();
    while day >= 1 {
        if let Some(date) = NaiveDate::from_ymd_opt(target_year, target_month as u32, day)
            && let Some(time) = NaiveTime::from_hms_opt(base.hour(), base.minute(), base.second())
        {
            return Some(NaiveDateTime::new(date, time));
        }
        day -= 1;
    }

    None
}

impl TryFrom<&KDbRow> for ToentDefi {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let start_timezone: Option<i64> = value.try_get(Self::START_TIMEZONE)?;
        let end_timezone: Option<i64> = value.try_get(Self::END_TIMEZONE)?;
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            event_defi: {
                let text: Text = value.try_get(Self::EVENT_DEFI)?;
                serde_json::from_str(text.as_str())?
            },
            start_tid: value.try_get(Self::START_TID)?,
            start_timezone: start_timezone.map(|v| v as isize),
            end_tid: value.try_get(Self::END_TID)?,
            end_timezone: end_timezone.map(|v| v as isize),
            tid: value.try_get(Self::TID)?,
            total_count: value.try_get(Self::TOTAL_COUNT)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
        })
    }
}

impl Curd for ToentDefi {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ToentDefi}
