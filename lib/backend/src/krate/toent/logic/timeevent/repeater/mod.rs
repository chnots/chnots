use chin_tools::AResult;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

pub(crate) mod endconditon;
pub(crate) mod interval;
pub(crate) mod timers;

use self::{endconditon::EndCondition, interval::TimeInterval};
use super::PossibleScore;
use crate::{
    enum_common_funcs,
    krate::toent::{EventBuilder, Words, dto::GuessElem},
};

use super::starts_any;

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash, Eq, Sequence)]
pub(crate) enum RepeatType {
    #[default]
    Once,
    RepeatEvent,
    RepeatTodo,
}

impl RepeatType {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            RepeatType::Once => ".",
            RepeatType::RepeatEvent => "*",
            RepeatType::RepeatTodo => "**",
        }
    }
}

pub enum Repeat {
    Interval(TimeInterval),
    // TODO, m3d4,w1,H3M4S10
}

enum_common_funcs!(RepeatType);

impl TryFrom<Option<&str>> for RepeatType {
    type Error = anyhow::Error;

    fn try_from(s: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(s) = s {
            let r = if s == RepeatType::Once.as_ref() {
                RepeatType::Once
            } else if s == RepeatType::RepeatEvent.as_ref() {
                RepeatType::RepeatEvent
            } else if s == RepeatType::RepeatTodo.as_ref() {
                RepeatType::RepeatTodo
            } else {
                anyhow::bail!("unable to deser this: {}", s)
            };

            Ok(r)
        } else {
            Ok(Self::default())
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct Repeater {
    interval: Option<(TimeInterval, RepeatType)>,
    alert: Option<TimeInterval>,
    end_cond: Option<EndCondition>,
}

impl Repeater {
    pub(crate) fn interval_start(seg: &str) -> bool {
        starts_any(seg, &["**", "*"])
    }

    pub(crate) fn alter_start(seg: &str) -> bool {
        starts_any(seg, &[","])
    }

    pub(crate) fn end_start(seg: &str) -> bool {
        starts_any(seg, &["="])
    }

    pub(crate) fn repeater_start(seg: &str) -> bool {
        Self::interval_start(seg) || Self::alter_start(seg) || Self::end_start(seg)
    }

    pub(crate) fn guess_from_segs(
        interval: Option<&Words>,
        end: Option<&Words>,
        alert: Option<&Words>,
    ) -> Vec<GuessElem<Self>> {
        if let Ok(segs) = Self::standard_from_segs(interval, end, alert) {
            vec![(segs, PossibleScore::Likely(255)).into()]
        } else {
            vec![]
        }
    }

    pub(crate) fn standard_from_segs(
        interval: Option<&Words>,
        end: Option<&Words>,
        alert: Option<&Words>,
    ) -> AResult<Self> {
        let interval = if let Some(e) = interval {
            let repeat_type = RepeatType::try_from(e.first().map(|e| e.text))?;
            Some((TimeInterval::try_from_standard(e)?, repeat_type))
        } else {
            None
        };

        let alert = if let Some(e) = alert {
            Some(TimeInterval::try_from_standard(e)?)
        } else {
            None
        };

        let end = if let Some(e) = end {
            Some(EndCondition::try_from_standard(e)?)
        } else {
            None
        };

        Ok(Repeater {
            interval,
            alert,
            end_cond: end,
        })
    }

    pub(crate) fn is_valid(&self) -> bool {
        true
    }

    pub(crate) fn standard_str(&self) -> String {
        let mut res = String::new();

        if let Some((interval, rt)) = &self.interval {
            res.push_str(rt.as_ref());
            res.push_str(interval.standard_string().as_str());
        }

        if let Some(alert) = &self.alert {
            res.push_str(" ,");
            res.push_str(alert.standard_string().as_str());
        }

        if let Some(end_cond) = &self.end_cond {
            res.push_str(end_cond.standard_string().as_str());
        }

        res
    }
}
