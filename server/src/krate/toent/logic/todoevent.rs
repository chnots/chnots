use std::str::FromStr;

use super::PossibleScore;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{de, Deserialize, Serialize};

use super::{EventBuilder, RawInputSegs};

use strum::{AsRefStr, EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, Debug, EnumString, AsRefStr, EnumIter, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub(crate) enum TodoEvent {
    Todo,
    Doing,
    Wait,
    Done,
    Cancel,
}

impl From<TodoEvent> for String {
    fn from(val: TodoEvent) -> Self {
        val.as_ref().to_string()
    }
}

impl Serialize for TodoEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for TodoEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let o: String = String::deserialize(deserializer)?;
        TodoEvent::from_str(o.to_ascii_uppercase().as_str()).map_err(serde::de::Error::custom)
    }
}

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
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let mut result = vec![];
        for ele in TodoEvent::iter() {
            let enum_str = ele.as_ref();
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

        Some(result)
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn try_from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        match gt.spans.first() {
            Some(s) => Ok(Self::from_str(s.text)?),
            None => {
                anyhow::bail!("There should at least one seg to deserialize TodoEnum")
            }
        }
    }

    fn standard_str(&self) -> String {
        self.as_ref().to_string()
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
