use std::collections::HashSet;

use chin_sql::{GenerateTableSchema, str_type::Text, time_type::TID};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

use crate::{
    impl_otid_support,
    krate::toent::{
        logic::todoevent::{TodoPriorityEnum, TodoStateEnum},
        timeevent::{
            TimeEvent,
            repeater::interval::TimeInterval,
            timeenum::{
                Timestamp, UtcWithOffset, UtcWithOffsetType, chinese::ChnTimeCalculator,
                westen::WesTime,
            },
        },
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

fn timeevent_to_sql(this: Option<ToentEventDefi>) -> Option<Text> {
    match this {
        Some(this) => serde_json::to_string(&this).ok().map(|e| e.into()),
        None => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentInst {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    #[gts_type = "i64"]
    pub alert_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_primary]
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

impl TryFrom<&KDbRow> for ToentInst {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let timezone: Option<i64> = value.try_get(Self::TIMEZONE)?;
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
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
        })
    }
}

impl Curd for ToentInst {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid, self.start_tid)
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
    pub event_defi: Option<ToentEventDefi>,

    pub todo_flag: bool,

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
pub(crate) struct ToentEventDefi {
    pub data: HashSet<TimeEvent>,
}

impl ToentEventDefi {
    pub(crate) fn start_time(&self) -> Option<UtcWithOffset> {
        self.data
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
        self.data
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

    pub(crate) fn to_po(&self, otid: TID, todo_flag: bool) -> ToentDefi {
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
            tid: TID::default(),
            todo_flag: todo_flag,
        }
    }
}

pub(crate) fn toent_defi_by_todo(otid: TID, todo_flag: bool) -> Option<ToentDefi> {
    if todo_flag {
        Some(ToentDefi {
            otid: otid,
            event_defi: None,
            todo_flag,
            start_tid: None,
            start_timezone: None,
            end_tid: None,
            end_timezone: None,
            total_count: None,
            tid: TID::default(),
        })
    } else {
        None
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
            todo_flag: value.try_get(Self::TODO_FLAG)?,
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
