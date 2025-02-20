pub mod base;
pub mod chinese;
pub mod westen;

use chrono::{DateTime, Utc};

use self::{chinese::ChnTime, westen::WesTime};
use super::PossibleScore;
use crate::toent::{EventBuilder, GuessType};

pub trait TimestampNow {
    fn now_time() -> Self;
    fn now_date() -> Self;
}

pub trait Timestamp {
    fn to_wes_timestamp(&self) -> DateTime<Utc>;

    fn calender_type(&self) -> &'static str;
}

#[derive(Clone, Debug)]

pub enum TimeEnum {
    Wes(WesTime),
    Chn(ChnTime),
}

impl EventBuilder for TimeEnum {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let mut result: Vec<(TimeEnum, PossibleScore)> = vec![];
        let wes: Vec<(Self, PossibleScore)> = WesTime::guess(input)
            .into_iter()
            .map(|(v, p)| (TimeEnum::Wes(v), p))
            .collect();

        let chn: Vec<(Self, PossibleScore)> = ChnTime::guess(input)
            .into_iter()
            .map(|(v, p)| (TimeEnum::Chn(v), p))
            .collect();

        result.extend(wes);
        result.extend(chn);

        result
    }

    fn is_valid(&self) -> bool {
        match self {
            TimeEnum::Wes(wes) => wes.is_valid(),
            TimeEnum::Chn(chn) => chn.is_valid(),
        }
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        if let Ok(res) = WesTime::from_standard(segs) {
            Ok(Self::Wes(res))
        } else if let Ok(res) = ChnTime::from_standard(segs) {
            Ok(Self::Chn(res))
        } else {
            anyhow::bail!(
                "Unable to parse it from chinese and westen calendar {:?}",
                segs
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
