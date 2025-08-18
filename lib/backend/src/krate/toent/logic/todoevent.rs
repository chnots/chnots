use crate::{enum_common_funcs, krate::toent::dto::GuessElem};

use super::PossibleScore;
use anyhow::Context;
use chin_tools::AResult;
use enum_iterator::{Sequence, all};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize, de};

use super::{EventBuilder, Words};

#[derive(Clone, Copy, Debug, PartialEq, Sequence, Default)]
pub(crate) enum TodoStateEnum {
    #[default]
    Todo,
    Doing,
    Wait,
    Done,
    Cancel,
}

impl PartialOrd for TodoStateEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_priority().cmp(&other.as_priority()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Sequence, Default)]
pub(crate) enum TodoPriorityEnum {
    A,
    B,
    #[default]
    C,
    D,
    E,
}

impl PartialOrd for TodoPriorityEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_priority().cmp(&other.as_priority()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub(crate) struct TodoEvent {
    pub state: TodoStateEnum,
    pub priority: Option<TodoPriorityEnum>,
}

impl PartialOrd for TodoEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.state.partial_cmp(&other.state) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.priority
            .unwrap_or_default()
            .partial_cmp(&other.priority.unwrap_or_default())
    }
}

impl TodoEvent {
    pub fn state(state: TodoStateEnum) -> Self {
        Self {
            state,
            priority: None,
        }
    }
}

impl TodoStateEnum {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            TodoStateEnum::Todo => "TODO",
            TodoStateEnum::Doing => "DOING",
            TodoStateEnum::Wait => "WAIT",
            TodoStateEnum::Done => "DONE",
            TodoStateEnum::Cancel => "CANCEL",
        }
    }

    pub fn as_priority(&self) -> i32 {
        match self {
            TodoStateEnum::Todo => 1,
            TodoStateEnum::Doing => 0,
            TodoStateEnum::Wait => 2,
            TodoStateEnum::Done => 3,
            TodoStateEnum::Cancel => 4,
        }
    }
}

impl<'de> Deserialize<'de> for TodoEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let o: String = String::deserialize(deserializer)?;
        let segs: Vec<&str> = o.split(' ').collect();
        let state_str = segs
            .first()
            .context("unable to extract state str")
            .and_then(|s| TodoStateEnum::try_from(*s))
            .map_err(|err| de::Error::custom(err.to_string()))?;
        let priority = match segs.get(1) {
            Some(s) => Some(
                TodoPriorityEnum::try_from(*s).map_err(|err| de::Error::custom(err.to_string()))?,
            ),
            None => None,
        };
        Ok(TodoEvent {
            state: state_str,
            priority,
        })
    }
}

impl TodoPriorityEnum {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            TodoPriorityEnum::A => "A",
            TodoPriorityEnum::B => "B",
            TodoPriorityEnum::C => "C",
            TodoPriorityEnum::D => "D",
            TodoPriorityEnum::E => "E",
        }
    }

    pub fn as_priority(&self) -> i32 {
        match self {
            TodoPriorityEnum::A => 0,
            TodoPriorityEnum::B => 1,
            TodoPriorityEnum::C => 2,
            TodoPriorityEnum::D => 3,
            TodoPriorityEnum::E => 4,
        }
    }
}

enum_common_funcs!(TodoStateEnum);
enum_common_funcs!(TodoPriorityEnum);

#[derive(FromPrimitive, ToPrimitive, Debug, Clone)]
pub(crate) enum TodoCreateType {
    Auto = 0,
    Manual = 1,
}

impl Serialize for TodoCreateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.to_i32().unwrap())
    }
}

impl<'de> Deserialize<'de> for TodoCreateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let o: i32 = i32::deserialize(deserializer)?;
        match TodoCreateType::from_i32(o) {
            Some(o) => Ok(o),
            None => Err(de::Error::custom("unable build TodoCreateType from i32")),
        }
    }
}

impl EventBuilder for TodoEvent {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let mut result = vec![];
        for ele in all::<TodoStateEnum>() {
            let enum_str = ele.as_static_str();
            let enum_len = enum_str.len();
            let upper_input = gt.original.to_uppercase();

            let distance = textdistance::str::damerau_levenshtein(enum_str, &upper_input);
            if distance < enum_len {
                let mut score = ((1. - distance as f32 / enum_len as f32) * 256.0) as u8;
                if enum_str.starts_with(&upper_input) {
                    score = score / 2 + 128;
                }
                if score < 128 {
                    continue;
                }
                result.push((ele, PossibleScore::Num(score)));
            }
        }

        Some(
            result
                .into_iter()
                .map(|(te, score)| {
                    (
                        TodoEvent {
                            state: te,
                            priority: None,
                        },
                        score,
                    )
                        .into()
                })
                .collect(),
        )
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let state = match gt.words.first() {
            Some(s) => TodoStateEnum::try_from(s.text.to_uppercase().as_str())?,
            None => {
                anyhow::bail!("There should at least one seg to deserialize TodoEnum")
            }
        };
        let priority = match gt.get(1) {
            Some(word) => Some(TodoPriorityEnum::try_from(
                word.text.to_uppercase().as_str(),
            )?),
            None => None,
        };
        Ok(Self { state, priority })
    }

    fn standard_string(&self) -> String {
        format!(
            "{}{}",
            self.state.as_static_str(),
            match self.priority {
                Some(p) => format!(" !{}", p.as_static_str()),
                None => "".to_owned(),
            }
        )
    }
}

impl Serialize for TodoEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.standard_string().serialize(serializer)
    }
}

#[cfg(test)]
mod test {

    use crate::krate::toent::EventBuilder;

    use super::TodoEvent;

    #[test]
    fn test() {
        println!("{:?}", TodoEvent::guess(&"done".into()));
    }
}
