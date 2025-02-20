use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{model::todo::TodoEvent, toent::retain_not_empty_parts};

use super::{timeevent::TimeEvent, EventBuilder, GuessType};
use super::PossibleScore;

#[derive(Clone, Debug)]
pub enum EventEnum {
    Time(TimeEvent),
    Todo(TodoEvent),
}

impl Serialize for EventEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.standard_str())
    }
}

impl<'de> Deserialize<'de> for EventEnum {
    fn deserialize<D>(deserializer: D) -> Result<EventEnum, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EEVisitor;

        impl<'a> serde::de::Visitor<'a> for EEVisitor {
            type Value = EventEnum;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("binding fields")
            }

            fn visit_str<E>(self, value: &str) -> Result<EventEnum, E>
            where
                E: de::Error,
            {
                EventEnum::from_standard(retain_not_empty_parts(value).as_slice())
                    .map_err(|e| de::Error::custom(e))
            }
        }

        deserializer.deserialize_str(EEVisitor)
    }
}

impl From<TimeEvent> for EventEnum {
    fn from(value: TimeEvent) -> Self {
        Self::Time(value)
    }
}

impl From<TodoEvent> for EventEnum {
    fn from(value: TodoEvent) -> Self {
        Self::Todo(value)
    }
}

impl EventBuilder for EventEnum {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let todo_vec = TodoEvent::guess(input);
        let time_vec = TimeEvent::guess(input);

        let mut result = vec![];
        result.extend(todo_vec.into_iter().map(|(v1, v2)| (v1.into(), v2)));
        result.extend(time_vec.into_iter().map(|(v1, v2)| (v1.into(), v2)));
        result
    }

    fn is_valid(&self) -> bool {
        match self {
            EventEnum::Time(v) => v.is_valid(),
            EventEnum::Todo(v) => v.is_valid(),
        }
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        let event: EventEnum;
        if let Ok(v) = TodoEvent::from_standard(segs) {
            event = v.into();
        } else {
            match TimeEvent::from_standard(segs) {
                Ok(v) => {
                    event = v.into();
                }
                Err(err) => {
                    anyhow::bail!(
                        "input {:?} could not be parsed by todo enum or time enum: {}",
                        segs,
                        err
                    );
                }
            }
        }
        Ok(event)
    }

    fn standard_str(&self) -> String {
        match self {
            EventEnum::Time(v) => v.standard_str(),
            EventEnum::Todo(v) => v.standard_str(),
        }
    }
}
