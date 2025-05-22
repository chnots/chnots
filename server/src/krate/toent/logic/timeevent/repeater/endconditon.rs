use super::interval::TimeInterval;
pub(crate) use super::timers::Times;
use super::PossibleScore;
use crate::krate::toent::{timeevent::timeenum::TimeEnum, EventBuilder, RawInputSegs};

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
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let input = gt.remove_first_prefix("=");

        let mut result = vec![];
        if let Some(v) = TimeEnum::guess(&input) {
            result.extend(v.into_iter().map(|e| (e.0.into(), e.1)));
        }
        if let Some(v) = TimeInterval::guess(&input) {
            result.extend(v.into_iter().map(|e| (e.0.into(), e.1)));
        }
        if let Some(v) = Times::guess(&input) {
            result.extend(v.into_iter().map(|e| (e.0.into(), e.1)));
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

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        if gt.is_empty() {
            anyhow::bail!("end condition should not be empty");
        }
        if gt.get(0).map_or(false, |e| !e.starts_with("=")) {
            anyhow::bail!("the end condition should start with =")
        }

        let gt = gt.remove_first_prefix("=");

        if let Ok(v) = TimeEnum::from_standard(&gt) {
            Ok(v.into())
        } else if let Ok(v) = TimeInterval::from_standard(&gt) {
            Ok(v.into())
        } else if let Ok(v) = Times::from_standard(&gt) {
            Ok(v.into())
        } else {
            anyhow::bail!("unable to parse it into end condition: {:?}", gt)
        }
    }

    fn standard_str(&self) -> String {
        let mut res = String::new();
        res.push('=');
        let v = match self {
            EndCondition::Times(v) => v.standard_str(),
            EndCondition::Interval(v) => v.standard_str(),
            EndCondition::Time(v) => v.standard_str(),
        };

        res.push_str(v.as_str());

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::krate::toent::{
        timeevent::{
            repeater::{endconditon::Times, interval::TimeInterval},
            timeenum::TimeEnum,
        },
        EventBuilder,
    };

    use super::EndCondition;

    #[test]
    fn test_standard_str() {
        assert_eq!(
            EndCondition::Interval(TimeInterval::from_standard(&"3d".into()).unwrap())
                .standard_str(),
            "=3d"
        );
        assert_eq!(
            EndCondition::Time(TimeEnum::from_standard(&"2025-12-25".into()).unwrap())
                .standard_str(),
            "=2025-12-25"
        );
        assert_eq!(
            EndCondition::Times(Times::from_standard(&"10t".into()).unwrap()).standard_str(),
            "=10t"
        );
    }

    #[test]
    fn test_interval() {
        assert!(
            EndCondition::guess(&"=10d".into())
                .unwrap()
                .get(0)
                .unwrap()
                .0
                == EndCondition::Interval(TimeInterval::from_standard(&"10d".into()).unwrap())
        );
    }

    #[test]
    fn test_time() {
        assert_eq!(
            EndCondition::guess(&"=2025-12-12".into())
                .unwrap()
                .get(0)
                .unwrap()
                .0,
            EndCondition::from_standard(&"=2025-12-12".into()).unwrap()
        );

        assert_eq!(
            EndCondition::guess(&"=2025-12-12 12:00:00".into())
                .unwrap()
                .get(0)
                .unwrap()
                .0,
            EndCondition::from_standard(&"=2025-12-12 12:00:00".into()).unwrap()
        );
    }
}
