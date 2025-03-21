pub mod base;
pub mod chinese;
pub mod westen;

use chrono::{DateTime, Utc};

use self::{chinese::ChnTime, westen::WesTime};
use super::{InputSegs, PossibleScore};
use crate::toent::{EventBuilder, RawInputSegs};

pub trait Timestamp {
    fn to_utc_timestamp(&self) -> DateTime<Utc>;

    fn calender_type(&self) -> &'static str;

    fn now_time() -> Self;
    fn now_date() -> Self;
}

#[derive(Clone, Debug, PartialEq)]

pub enum TimeEnum {
    Wes(WesTime),
    Chn(ChnTime),
}

impl From<ChnTime> for TimeEnum {
    fn from(value: ChnTime) -> Self {
        Self::Chn(value)
    }
}

impl EventBuilder for TimeEnum {
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let mut result: Vec<(TimeEnum, PossibleScore)> = vec![];
        if gt.len() <= 2 {
            result.push((ChnTime::now_date().into(), PossibleScore::Maybe(0)));
            result.push((ChnTime::now_time().into(), PossibleScore::Maybe(0)));
        }

        if let Some(vs) = WesTime::guess(gt) {
            let wes: Vec<(Self, PossibleScore)> =
                vs.into_iter().map(|(v, p)| (TimeEnum::Wes(v), p)).collect();
            result.extend(wes);
        }

        if let Some(vs) = ChnTime::guess(gt) {
            let chn: Vec<(Self, PossibleScore)> =
                vs.into_iter().map(|(v, p)| (TimeEnum::Chn(v), p)).collect();

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

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        if let Ok(res) = WesTime::from_standard(gt) {
            Ok(Self::Wes(res))
        } else if let Ok(res) = ChnTime::from_standard(gt) {
            Ok(Self::Chn(res))
        } else {
            anyhow::bail!(
                "Unable to parse it from chinese and westen calendar {:?}",
                gt
            )
        }
    }

    fn standard_str(&self) -> String {
        match self {
            TimeEnum::Wes(wes) => wes.standard_str(),
            TimeEnum::Chn(chn) => chn.standard_str(),
        }
    }
}
