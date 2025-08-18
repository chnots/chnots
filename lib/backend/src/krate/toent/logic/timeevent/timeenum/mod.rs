pub(crate) mod base;
pub(crate) mod chinese;
pub(crate) mod westen;

use chin_tools::AResult;
use chrono::{DateTime, Utc};

use self::{chinese::ChnTime, westen::WesTime};
use super::PossibleScore;
use crate::krate::toent::{EventBuilder, Words, dto::GuessElem};

pub(crate) trait Timestamp {
    fn to_utc_timestamp(&self) -> DateTime<Utc>;

    fn calender_type(&self) -> &'static str;

    fn now_time() -> Self;
    fn now_date() -> Self;
}

#[derive(Clone, Debug, PartialEq)]

pub(crate) enum TimeEnum {
    Wes(WesTime),
    Chn(ChnTime),
}

impl From<ChnTime> for TimeEnum {
    fn from(value: ChnTime) -> Self {
        Self::Chn(value)
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
                .map(|GuessElem { toent: v, score: p }| (TimeEnum::Wes(v), p).into())
                .collect();
            result.extend(wes);
        }

        if let Some(vs) = ChnTime::guess(gt) {
            let chn: Vec<GuessElem<Self>> = vs
                .into_iter()
                .map(|GuessElem { toent: v, score: p }| (TimeEnum::Chn(v), p).into())
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
