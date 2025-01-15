use std::str::FromStr;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{de, Deserialize, Serialize};
use strum::IntoEnumIterator;


use crate::model::{score::PossibleScore, todo::TodoEvent};

use super::{EventBuilder, GuessType};

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
        TodoEvent::from_str(o.to_ascii_uppercase().as_str())
            .map_err(|err| serde::de::Error::custom(err))
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone)]
pub enum TodoCreateType {
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
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let mut result = vec![];
        for ele in TodoEvent::iter() {
            let enum_str = ele.as_ref();
            let enum_len = enum_str.len();
            let upper_input = input.original.to_uppercase();

            let distance = distance::levenshtein(enum_str, &upper_input);
            if distance < enum_len {
                let mut pos = ((1. - distance as f32 / enum_len as f32) * 256.0) as u8;
                if enum_str.starts_with(&upper_input) {
                    pos = pos / 2 + 128;
                }
                if pos < 128 {
                    continue;
                }
                result.push((ele, PossibleScore::Num(pos)));
            }
        }

        result
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        match segs.get(0) {
            Some(s) => Ok(Self::from_str(s)?),
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

    use crate::toent::EventBuilder;

    use super::TodoEvent;

    #[test]
    fn test() {
        println!("{:?}", TodoEvent::guess(&"done".into()));
    }
}
