use once_cell::sync::Lazy;
use regex::Regex;

use crate::toent::{timeevent::timeenum::TimeEnum, EventBuilder, GuessType};
use super::PossibleScore;
use super::interval::TimeInterval;

#[derive(Clone, Debug)]

pub struct Times {
    count: u32,
}

static TIMES_REGEX: Lazy<Regex> = regex_static::lazy_regex!(r"^(\d+)t$");

impl EventBuilder for Times {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        match Self::from_standard(&input) {
            Ok(v) => vec![(v, PossibleScore::Likely(100))],
            Err(_) => vec![],
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        if segs.len() != 1 {
            anyhow::bail!("Times segs' count Should be 1: {:?}", segs);
        }

        match TIMES_REGEX.captures(segs[0]).map(|e| e.get(1)) {
            Some(Some(v)) => Ok(Self {
                count: u32::from_str_radix(v.as_str(), 10)?,
            }),
            _ => {
                anyhow::bail!("unable to parse it: {:?}", segs)
            }
        }
    }

    fn standard_str(&self) -> String {
        format!("{}t", self.count)
    }
}

#[derive(Clone, Debug)]
pub enum EndCondition {
    Times(Times),
    Interval(TimeInterval),
    Time(TimeEnum),
}

impl From<Times> for EndCondition {
    fn from(value: Times) -> Self {
        Self::Times(value)
    }
}
impl From<TimeInterval> for EndCondition {
    fn from(value: TimeInterval) -> Self {
        Self::Interval(value)
    }
}
impl From<TimeEnum> for EndCondition {
    fn from(value: TimeEnum) -> Self {
        Self::Time(value)
    }
}

impl EventBuilder for EndCondition {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let mut value = vec![];
        value.extend(
            TimeEnum::guess(input)
                .into_iter()
                .map(|e| (e.0.into(), e.1)),
        );
        value.extend(
            TimeInterval::guess(input)
                .into_iter()
                .map(|e| (e.0.into(), e.1)),
        );
        value.extend(Times::guess(input).into_iter().map(|e| (e.0.into(), e.1)));
        value
    }

    fn is_valid(&self) -> bool {
        match self {
            EndCondition::Times(v) => v.is_valid(),
            EndCondition::Interval(v) => v.is_valid(),
            EndCondition::Time(v) => v.is_valid(),
        }
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        if segs.is_empty() {
            anyhow::bail!("end condition should not be empty");
        }
        if let Ok(v) = TimeEnum::from_standard(segs) {
            Ok(v.into())
        } else if let Ok(v) = TimeInterval::from_standard(segs) {
            Ok(v.into())
        } else if let Ok(v) = Times::from_standard(segs) {
            Ok(v.into())
        } else {
            anyhow::bail!("unable to parse it into end condition: {:?}", segs)
        }
    }

    fn standard_str(&self) -> String {
        let mut res = String::new();
        let v = match self {
            EndCondition::Times(v) => v.standard_str(),
            EndCondition::Interval(v) => v.standard_str(),
            EndCondition::Time(v) => v.standard_str(),
        };

        res.push_str(v.as_str());

        res
    }
}
