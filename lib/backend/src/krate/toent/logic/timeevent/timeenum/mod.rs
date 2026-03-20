pub(crate) mod base;
pub(crate) mod chinese;
pub(crate) mod chinese_cal_calc;
pub(crate) mod westen;

use std::ops::{Add, Sub};

use chin_sql::time_type::TID;
use chin_tools::AResult;
use chrono::{Datelike, Days, Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use self::westen::WesTime;
use super::PossibleScore;
use crate::krate::toent::{
    EventBuilder, Words,
    dto::GuessElem,
    timeevent::{
        repeater::interval::TimeInterval,
        timeenum::{base::BaseDateTime, chinese::ChnTime},
    },
};

pub(crate) trait Timestamp {
    fn to_utc_timestamp(&self) -> AResult<UtcWithOffsetType>;

    fn calender_type(&self) -> &'static str;

    fn now_time() -> Self;
    fn now_date() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) enum TimeEnum {
    Wes(WesTime),
    Chn(ChnTime),
}

impl From<TID> for TimeEnum {
    fn from(tid: TID) -> Self {
        let utc = tid.as_utc().naive_utc();
        TimeEnum::Wes(WesTime {
            local_minus_utc: None,
            timestamp: BaseDateTime::from(utc),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UtcWithOffset {
    utc: TID,
    local_minus_utc: i32,
}

impl From<TID> for UtcWithOffset {
    fn from(value: TID) -> Self {
        Self {
            utc: value,
            local_minus_utc: 0,
        }
    }
}

fn naive_datetime_add_interval(
    base: Option<chrono::NaiveDateTime>,
    interval: TimeInterval,
) -> Option<chrono::NaiveDateTime> {
    let Some(base) = base else {
        return None;
    };
    let years = interval.date.year.unwrap_or(0);
    let months = interval.date.month.unwrap_or(0);

    // 把“年、月增量”统一折算成总月数，便于处理跨年与负数月份。
    let month0 = i64::from(base.month0());
    let total_months =
        i64::from(base.year()) * 12 + month0 + i64::from(years) * 12 + i64::from(months);

    // 使用欧几里得除法，保证负数月份也能得到正确的年/月。
    let new_year_i64 = total_months.div_euclid(12);
    let new_month0_i64 = total_months.rem_euclid(12);

    let new_year = i32::try_from(new_year_i64).ok()?;
    let new_month = u32::try_from(new_month0_i64 + 1).ok()?;

    // 先定位到目标年月的第一天，再计算该月最后一天。
    let first_day = NaiveDate::from_ymd_opt(new_year, new_month, 1)?;
    let next_month_first_day = if new_month == 12 {
        NaiveDate::from_ymd_opt(new_year.checked_add(1)?, 1, 1)?
    } else {
        NaiveDate::from_ymd_opt(new_year, new_month + 1, 1)?
    };

    let last_day = (next_month_first_day - Days::new(1)).day();
    // 若原日期的“日”在目标月不存在（如 31 号），自动夹到目标月月末。
    let target_day = base.day().min(last_day);
    let target_date = first_day.with_day(target_day)?;

    // 保留原始时分秒后，再叠加周/日/时/分/秒增量。
    let with_ym = target_date.and_time(base.time());
    let day_offset =
        i64::from(interval.date.day.unwrap_or(0)) + i64::from(interval.week.unwrap_or(0)) * 7;

    Some(
        with_ym
            + Duration::days(day_offset)
            + Duration::hours(i64::from(interval.time.hour.unwrap_or(0)))
            + Duration::minutes(i64::from(interval.time.minute.unwrap_or(0)))
            + Duration::seconds(i64::from(interval.time.second.unwrap_or(0))),
    )
}

fn naive_datetime_sub_interval(
    base: Option<chrono::NaiveDateTime>,
    interval: TimeInterval,
) -> Option<chrono::NaiveDateTime> {
    let mut negated = interval;
    negated.date.year = (-1 * interval.date.year.unwrap_or(0)).into();
    negated.date.month = (-1 * interval.date.month.unwrap_or(0)).into();
    negated.date.day = (-1 * interval.date.day.unwrap_or(0)).into();
    negated.week = (-1 * interval.week.unwrap_or(0)).into();
    negated.time.hour = (-1 * interval.time.hour.unwrap_or(0)).into();
    negated.time.minute = (-1 * interval.time.minute.unwrap_or(0)).into();
    negated.time.second = (-1 * interval.time.second.unwrap_or(0)).into();

    naive_datetime_add_interval(base, negated)
}

impl Add<TimeInterval> for UtcWithOffset {
    type Output = Option<UtcWithOffset>;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        let start = self;

        let offset = start.local_minus_utc();
        let utc_dt = start.utc().as_utc().naive_utc();
        let local_dt = utc_dt + Duration::seconds(i64::from(offset));

        let result = match naive_datetime_add_interval(Some(local_dt), rhs) {
            Some(v) => v,
            None => return None,
        };

        Some(UtcWithOffset {
            utc: match result.and_utc().timestamp_micros().try_into() {
                Ok(v) => v,
                Err(_) => return None,
            },
            local_minus_utc: self.local_minus_utc,
        })
    }
}

impl Sub<TimeInterval> for UtcWithOffset {
    type Output = Option<UtcWithOffset>;

    fn sub(self, rhs: TimeInterval) -> Self::Output {
        let start = self;

        let offset = start.local_minus_utc();
        let utc_dt = start.utc().as_utc().naive_utc();
        let local_dt = utc_dt - Duration::seconds(i64::from(offset));

        let result = match naive_datetime_sub_interval(Some(local_dt), rhs) {
            Some(v) => v,
            None => return None,
        };

        Some(UtcWithOffset {
            utc: match result.and_utc().timestamp_micros().try_into() {
                Ok(v) => v,
                Err(_) => return None,
            },
            local_minus_utc: self.local_minus_utc,
        })
    }
}

pub enum UtcWithOffsetType {
    Period {
        start: UtcWithOffset,
        end: UtcWithOffset,
    },
    Point(UtcWithOffset),
}

impl UtcWithOffsetType {
    pub fn start(&self) -> UtcWithOffset {
        match self {
            UtcWithOffsetType::Period { start, end: _ } => *start,
            UtcWithOffsetType::Point(utc_with_offset) => *utc_with_offset,
        }
    }

    pub fn end(&self) -> UtcWithOffset {
        match self {
            UtcWithOffsetType::Period { start: _, end } => *end,
            UtcWithOffsetType::Point(utc_with_offset) => *utc_with_offset,
        }
    }
}

impl UtcWithOffset {
    pub(crate) fn new(utc: TID, local_minus_utc: i32) -> Self {
        Self {
            utc,
            local_minus_utc,
        }
    }

    pub fn from_utc(tid: TID) -> Self {
        Self {
            utc: tid,
            local_minus_utc: Local::now().offset().local_minus_utc(),
        }
    }

    #[inline]
    pub(crate) fn utc(&self) -> TID {
        self.utc
    }

    #[inline]
    pub(crate) fn local_minus_utc(&self) -> i32 {
        self.local_minus_utc
    }
}

impl TimeEnum {
    pub fn to_utc_timestamp(&self) -> AResult<UtcWithOffsetType> {
        match self {
            TimeEnum::Wes(wes_time) => wes_time.to_utc_timestamp(),
            TimeEnum::Chn(chn_time) => chn_time.to_utc_timestamp(),
        }
    }
}

impl From<ChnTime> for TimeEnum {
    fn from(value: ChnTime) -> Self {
        Self::Chn(value)
    }
}

impl From<WesTime> for TimeEnum {
    fn from(value: WesTime) -> Self {
        Self::Wes(value)
    }
}

impl Add<TimeInterval> for TimeEnum {
    type Output = Option<TimeEnum>;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        match self {
            TimeEnum::Wes(wes_time) => (wes_time + rhs).map(|w| TimeEnum::Wes(w)),
            TimeEnum::Chn(chn_time) => (chn_time + rhs).map(|c| TimeEnum::Chn(c)),
        }
    }
}

impl EventBuilder for TimeEnum {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let mut result: Vec<GuessElem<TimeEnum>> = vec![];
        if gt.original.len() <= 2 {
            result.push((ChnTime::now_date().into(), PossibleScore::Maybe(0)).into());
            result.push((ChnTime::now_time().into(), PossibleScore::Maybe(0)).into());
        }

        if let Some(vs) = WesTime::guess(gt) {
            let wes: Vec<GuessElem<Self>> = vs
                .into_iter()
                .map(
                    |GuessElem {
                         timestamp: v,
                         score: p,
                     }| (TimeEnum::Wes(v), p).into(),
                )
                .collect();
            result.extend(wes);
        }

        if let Some(vs) = ChnTime::guess(gt) {
            let chn: Vec<GuessElem<Self>> = vs
                .into_iter()
                .map(
                    |GuessElem {
                         timestamp: v,
                         score: p,
                     }| (TimeEnum::Chn(v), p).into(),
                )
                .collect();

            result.extend(chn);
        }

        Some(result)
    }

    fn is_valid(&self) -> bool {
        match self {
            TimeEnum::Wes(wes) => wes.is_valid(),
            TimeEnum::Chn(chn) => chn.is_valid(),
        }
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        if let Ok(res) = WesTime::try_from_standard(gt) {
            Ok(Self::Wes(res))
        } else if let Ok(res) = ChnTime::try_from_standard(gt) {
            Ok(Self::Chn(res))
        } else {
            anyhow::bail!(
                "Unable to parse it from chinese and westen calendar {:?}",
                gt
            )
        }
    }

    fn standard_string(&self) -> String {
        match self {
            TimeEnum::Wes(wes) => wes.standard_string(),
            TimeEnum::Chn(chn) => chn.standard_string(),
        }
    }
}
