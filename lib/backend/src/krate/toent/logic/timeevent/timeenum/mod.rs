pub(crate) mod base;
pub(crate) mod chinese;
pub(crate) mod chinese_cal_calc;
pub(crate) mod westen;

use std::ops::Add;

use chin_sql::time_type::TID;
use chin_tools::AResult;
use chrono::Local;
use serde::{Deserialize, Serialize};

use self::westen::WesTime;
use super::PossibleScore;
use crate::krate::toent::{
    EventBuilder, Words,
    dto::GuessElem,
    timeevent::{repeater::interval::TimeInterval, timeenum::chinese::ChnTime},
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UtcWithOffset {
    utc: TID,
    local_minus_utc: i32,
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
}

impl UtcWithOffsetType {
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

    pub(crate) fn utc(&self) -> TID {
        self.utc
    }

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
    type Output = TimeEnum;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        match self {
            TimeEnum::Wes(wes_time) => TimeEnum::Wes(wes_time + rhs),
            TimeEnum::Chn(chn_time) => TimeEnum::Chn(chn_time + rhs),
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
