use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};


use super::todoevent::TodoEvent;
use super::PossibleScore;
use super::{timeevent::TimeEvent, EventBuilder, RawInputSegs};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EventEnum {
    Time(Box<TimeEvent>),
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
                EventEnum::from_standard(&RawInputSegs::from(value))
                    .map_err(|e| de::Error::custom(e))
            }
        }

        deserializer.deserialize_str(EEVisitor)
    }
}

impl From<TimeEvent> for EventEnum {
    fn from(value: TimeEvent) -> Self {
        Self::Time(value.into())
    }
}

impl From<TodoEvent> for EventEnum {
    fn from(value: TodoEvent) -> Self {
        Self::Todo(value)
    }
}

impl EventBuilder for EventEnum {
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let mut result = vec![];
        if gt.is_empty() {
            result.push((TodoEvent::Todo.into(), PossibleScore::Maybe(0)));
            result.push((TimeEvent::now().into(), PossibleScore::Maybe(0)));
        }
        if let Some(todo_vec) = TodoEvent::guess(gt) {
            result.extend(
                todo_vec
                    .into_iter()
                    .map(|(event, score)| (event.into(), score)),
            );
        }
        if let Some(time_vec) = TimeEvent::guess(gt) {
            result.extend(
                time_vec
                    .into_iter()
                    .map(|(event, score)| (event.into(), score)),
            );
        }
        Some(result)
    }

    fn is_valid(&self) -> bool {
        match self {
            EventEnum::Time(v) => v.is_valid(),
            EventEnum::Todo(v) => v.is_valid(),
        }
    }

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let event: EventEnum;
        if let Ok(todo_event) = TodoEvent::from_standard(gt) {
            event = todo_event.into();
        } else if let Ok(time_event) = TimeEvent::from_standard(gt) {
            event = time_event.into();
        } else {
            anyhow::bail!("input {gt:?} could not be parsed by todo enum or time enum",);
        };
        Ok(event)
    }

    fn standard_str(&self) -> String {
        match self {
            EventEnum::Time(v) => v.standard_str().trim().to_owned(),
            EventEnum::Todo(v) => v.standard_str().trim().to_owned(),
        }
    }
}
