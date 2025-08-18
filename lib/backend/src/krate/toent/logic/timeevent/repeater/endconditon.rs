use chin_tools::AResult;

use super::interval::TimeInterval;
pub(crate) use super::timers::Times;
use crate::krate::toent::{
    EventBuilder, Words,
    dto::{GuessElem, toent2},
    timeevent::timeenum::TimeEnum,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EndCondition {
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
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let input = gt.remove_first_prefix("=");

        let mut result = vec![];
        if let Some(v) = TimeEnum::guess(&input) {
            result.extend(v.into_iter().map(toent2));
        }
        if let Some(v) = TimeInterval::guess(&input) {
            result.extend(v.into_iter().map(toent2));
        }
        if let Some(v) = Times::guess(&input) {
            fn fun_name(e: GuessElem<Times>) -> GuessElem<EndCondition> {
                toent2(e)
            }
            result.extend(v.into_iter().map(fun_name));
        }
        Some(result)
    }

    fn is_valid(&self) -> bool {
        match self {
            EndCondition::Times(v) => v.is_valid(),
            EndCondition::Interval(v) => v.is_valid(),
            EndCondition::Time(v) => v.is_valid(),
        }
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        if gt.is_empty() {
            anyhow::bail!("end condition should not be empty");
        }
        if gt.first().is_some_and(|e| !e.starts_with("=")) {
            anyhow::bail!("the end condition should start with =")
        }

        let gt = gt.remove_first_prefix("=");

        if let Ok(v) = TimeEnum::try_from_standard(&gt) {
            Ok(v.into())
        } else if let Ok(v) = TimeInterval::try_from_standard(&gt) {
            Ok(v.into())
        } else if let Ok(v) = Times::try_from_standard(&gt) {
            Ok(v.into())
        } else {
            anyhow::bail!("unable to parse it into end condition: {:?}", gt)
        }
    }

    fn standard_string(&self) -> String {
        let mut res = String::new();
        res.push('=');
        let v = match self {
            EndCondition::Times(v) => v.standard_string(),
            EndCondition::Interval(v) => v.standard_string(),
            EndCondition::Time(v) => v.standard_string(),
        };

        res.push_str(v.as_str());

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::krate::toent::{
        EventBuilder,
        logic::timeevent::timeenum::westen::WesTime,
        timeevent::{
            repeater::{endconditon::Times, interval::TimeInterval},
            timeenum::TimeEnum,
        },
    };

    use super::EndCondition;

    #[test]
    fn test_standard_str() {
        assert_eq!(
            EndCondition::Interval(TimeInterval::try_from_standard(&"3d".into()).unwrap())
                .standard_string(),
            "=3d"
        );
        assert_eq!(
            EndCondition::Time(TimeEnum::try_from_standard(&"2025-12-25".into()).unwrap())
                .standard_string(),
            "=2025-12-25"
        );
        assert_eq!(
            EndCondition::Times(Times::try_from_standard(&"10t".into()).unwrap()).standard_string(),
            "=10t"
        );
    }

    #[test]
    fn test_interval() {
        assert_eq!(
            EndCondition::try_from_standard(&"=10d".into()).unwrap(),
            EndCondition::Interval(TimeInterval::try_from_standard(&"10d".into()).unwrap())
        );
    }

    #[test]
    fn test_time() {
        assert_eq!(
            EndCondition::Time(TimeEnum::Wes(
                WesTime::try_from_standrd_str("2025-12-12").unwrap()
            )),
            EndCondition::try_from_standard(&"=2025-12-12".into()).unwrap()
        );

        assert_eq!(
            EndCondition::Time(TimeEnum::Wes(
                WesTime::try_from_standrd_str("2025-12-12 12:00:00").unwrap()
            )),
            EndCondition::try_from_standard(&"=2025-12-12 12:00:00".into()).unwrap()
        );
    }
}
