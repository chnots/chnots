use std::{fmt::Display, ops::Deref};

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

use crate::{
    model::score::PossibleScore,
    toent::{EventBuilder, GuessType},
};

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct Unit(Option<i32>);

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(v) => {
                write!(f, "{:02}", v)
            }
            None => {
                write!(f, "{}", "?")
            }
        }
    }
}

impl From<i32> for Unit {
    fn from(value: i32) -> Self {
        Self(Some(value))
    }
}

impl From<u32> for Unit {
    fn from(value: u32) -> Self {
        Self(Some(value as i32))
    }
}

impl From<&str> for Unit {
    fn from(value: &str) -> Self {
        match i32::from_str_radix(value, 10) {
            Ok(i) => Unit(Some(i)),
            Err(_) => Unit(None),
        }
    }
}

impl From<String> for Unit {
    fn from(value: String) -> Self {
        match i32::from_str_radix(value.as_str(), 10) {
            Ok(i) => Unit(Some(i)),
            Err(_) => Unit(None),
        }
    }
}

impl From<Option<&&str>> for Unit {
    fn from(value: Option<&&str>) -> Self {
        match value {
            Some(v) => Unit::from(*v),
            None => Unit(None),
        }
    }
}

impl From<Option<String>> for Unit {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Unit::from(v),
            None => Unit(None),
        }
    }
}

impl Deref for Unit {
    type Target = Option<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseTime {
    pub year: Unit,
    pub month: Unit,
    pub day: Unit,
    pub hour: Unit,
    pub minute: Unit,
    pub second: Unit,
}

pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Week,
}

impl BaseTime {
    pub fn with_year(mut self, year: i32) -> Self {
        self.year = year.into();
        self
    }

    pub fn with_month(mut self, month: i32) -> Self {
        self.month = month.into();
        self
    }

    pub fn with_day(mut self, day: i32) -> Self {
        self.day = day.into();
        self
    }
    pub fn with_hour(mut self, hour: i32) -> Self {
        self.hour = hour.into();
        self
    }
    pub fn with_minute(mut self, minute: i32) -> Self {
        self.minute = minute.into();
        self
    }

    pub fn with_second(mut self, second: i32) -> Self {
        self.second = second.into();
        self
    }
}

macro_rules! all_some {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_some() && all_some!($($tail),*)
    };
}

macro_rules! all_none {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_none() && all_none!($($tail),*)
    };
}

impl EventBuilder for BaseTime {
    fn is_valid(&self) -> bool {
        (all_some!(self.year, self.month)
            && all_none!(self.day, self.hour, self.minute, self.second))
            || (all_some!(self.year, self.month, self.day)
                && all_none!(self.hour, self.minute, self.second))
            || (all_some!(self.year, self.month, self.day, self.hour,)
                && all_none!(self.minute, self.second))
            || (all_some!(self.year, self.month, self.day, self.hour, self.minute,)
                && all_none!(self.second))
            || (all_some!(
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second
            ))
    }

    fn from_standard(standard: &[&str]) -> anyhow::Result<Self> {
        if standard.len() != 2 && standard.len() != 1 {
            anyhow::bail!(
                "There should be like '2022-12-02' '20:00:00', found {:?}",
                standard
            )
        } else {
            let mut year = None::<String>;
            let mut month = None::<String>;
            let mut day = None::<String>;
            let mut hour = None::<String>;
            let mut minute = None::<String>;
            let mut second = None::<String>;

            for (id, value) in standard[0].split("-").enumerate() {
                match id {
                    0 => year = Some(value.into()),
                    1 => month = Some(value.into()),
                    2 => day = Some(value.into()),
                    _ => {}
                }
            }

            if standard.len() == 2 {
                for (id, value) in standard[1].split(":").enumerate() {
                    match id {
                        0 => hour = Some(value.into()),
                        1 => minute = Some(value.into()),
                        2 => second = Some(value.into()),
                        _ => {}
                    }
                }
            }

            let bts = BaseTime {
                year: year.into(),
                month: month.into(),
                day: day.into(),
                hour: hour.into(),
                minute: minute.into(),
                second: second.into(),
            };

            if bts.is_valid() {
                Ok(bts)
            } else {
                anyhow::bail!("unable to parse timestamp: {:?}", standard)
            }
        }
    }

    fn standard_str(&self) -> String {
        if self.second.is_some() {
            format!(
                "{}-{}-{} {}:{}:{}",
                self.year, self.month, self.day, self.hour, self.minute, self.second
            )
        } else if self.minute.is_some() {
            format!(
                "{}-{}-{} {}:{}",
                self.year, self.month, self.day, self.hour, self.minute
            )
        } else if self.hour.is_some() {
            format!("{}-{}-{} {}", self.year, self.month, self.day, self.hour)
        } else if self.day.is_some() {
            format!("{}-{}-{}", self.year, self.month, self.day)
        } else {
            format!("{}-{}", self.year, self.month)
        }
    }

    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        match Self::from_standard(&input.segs) {
            Ok(base) => {
                vec![(base, PossibleScore::Likely(100))]
            }
            Err(_) => {
                vec![]
            }
        }
    }
}

impl Default for BaseTime {
    fn default() -> Self {
        Self {
            year: Unit::default(),
            month: Unit::default(),
            day: Unit::default(),
            hour: Unit::default(),
            minute: Unit::default(),
            second: Unit::default(),
        }
    }
}

impl From<NaiveDateTime> for BaseTime {
    fn from(value: NaiveDateTime) -> Self {
        BaseTime {
            year: value.year().into(),
            month: value.month().into(),
            day: value.day().into(),
            hour: value.hour().into(),
            minute: value.minute().into(),
            second: value.second().into(),
        }
    }
}

impl From<NaiveDate> for BaseTime {
    fn from(value: NaiveDate) -> Self {
        BaseTime {
            year: value.year().into(),
            month: value.month().into(),
            day: value.day().into(),
            ..Default::default()
        }
    }
}

impl From<NaiveTime> for BaseTime {
    fn from(value: NaiveTime) -> Self {
        BaseTime {
            hour: value.hour().into(),
            minute: value.minute().into(),
            second: value.second().into(),
            ..Default::default()
        }
    }
}

pub fn convert_time_to_secs(input: &str, unit: TimeUnit) -> anyhow::Result<i32> {
    match unit {
        TimeUnit::Minute => {
            let time: Vec<&str> = input
                .trim_start_matches(|e| e == '+' || e == '-')
                .split(":")
                .collect();

            Ok(i32::from_str_radix(time[0], 10)? * 3600 + i32::from_str_radix(time[1], 10)? * 60)
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::toent::{timeevent::timeenum::base::BaseTime, EventBuilder};

    #[test]
    fn test_all() {
        // println!("{:?}", BaseTimestamp::from_standard(&["asdasd"]));
        // println!("{:?}", BaseTimestamp::from_standard(&["12"]));
        // println!("{:?}", BaseTimestamp::from_standard(&["12:03"]));
        println!("{:?}", BaseTime::from_standard(&["12-03"]));
        println!("{:?}", BaseTime::from_standard(&["12-03-04"]));
        println!("{:?}", BaseTime::from_standard(&["12-03-04", "12"]));
        println!("{:?}", BaseTime::from_standard(&["12-03-04", "12:12"]));
        println!("{:?}", BaseTime::from_standard(&["12-03-04", "12:12:12"]));
        // println!("{:?}", BaseTimestamp::from_standard(&["12-03-04", "12:12:12:q23e"]));
        // println!("{:?}", BaseTimestamp::from_standard(&["12-03-04", "12:12:12:q23e", "asdasd"]));
    }
}
