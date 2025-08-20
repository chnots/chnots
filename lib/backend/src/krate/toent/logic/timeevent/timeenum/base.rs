use std::{fmt::Display, ops::Deref};

use chin_tools::AResult;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

use crate::krate::toent::{EventBuilder, Words, dto::GuessElem};

use super::PossibleScore;

#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq)]
pub(crate) struct NoneOrI32(Option<i32>);

impl Display for NoneOrI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(v) => {
                write!(f, "{v:02}")
            }
            None => {
                write!(f, "?")
            }
        }
    }
}

impl From<i32> for NoneOrI32 {
    fn from(value: i32) -> Self {
        Self(Some(value))
    }
}

impl From<u32> for NoneOrI32 {
    fn from(value: u32) -> Self {
        Self(Some(value as i32))
    }
}

impl From<&str> for NoneOrI32 {
    fn from(value: &str) -> Self {
        match value.parse::<i32>() {
            Ok(i) => NoneOrI32(Some(i)),
            Err(_) => NoneOrI32(None),
        }
    }
}

impl From<String> for NoneOrI32 {
    fn from(value: String) -> Self {
        match value.as_str().parse::<i32>() {
            Ok(i) => NoneOrI32(Some(i)),
            Err(_) => NoneOrI32(None),
        }
    }
}

impl<T> From<Option<T>> for NoneOrI32
where
    T: Into<NoneOrI32>,
{
    fn from(value: Option<T>) -> Self {
        value.map_or(NoneOrI32(None), |v| v.into())
    }
}
impl Deref for NoneOrI32 {
    type Target = Option<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct BaseTime {
    pub(crate) year: NoneOrI32,
    pub(crate) month: NoneOrI32,
    pub(crate) day: NoneOrI32,
    pub(crate) hour: NoneOrI32,
    pub(crate) minute: NoneOrI32,
    pub(crate) second: NoneOrI32,
}

pub(crate) enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Week,
}

impl BaseTime {
    pub(crate) fn with_year(mut self, year: i32) -> Self {
        self.year = year.into();
        self
    }

    pub(crate) fn with_month(mut self, month: i32) -> Self {
        self.month = month.into();
        self
    }

    pub(crate) fn with_day(mut self, day: i32) -> Self {
        self.day = day.into();
        self
    }
    pub(crate) fn with_hour(mut self, hour: i32) -> Self {
        self.hour = hour.into();
        self
    }
    pub(crate) fn with_minute(mut self, minute: i32) -> Self {
        self.minute = minute.into();
        self
    }

    pub(crate) fn with_second(mut self, second: i32) -> Self {
        self.second = second.into();
        self
    }

    fn base_time2(
        year: Option<i32>,
        month: Option<i32>,
        day: Option<i32>,
        hour: Option<i32>,
        minute: Option<i32>,
        second: Option<i32>,
    ) -> BaseTime {
        BaseTime {
            year: year.into(),
            month: month.into(),
            day: day.into(),
            hour: hour.into(),
            minute: minute.into(),
            second: second.into(),
        }
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

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let standard = &gt.words;
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

            for (tid, value) in standard[0].split("-").enumerate() {
                match tid {
                    0 => year = Some(value.into()),
                    1 => month = Some(value.into()),
                    2 => day = Some(value.into()),
                    _ => {}
                }
            }

            if standard.len() == 2 {
                for (tid, value) in standard[1].split(":").enumerate() {
                    match tid {
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

    fn standard_string(&self) -> String {
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

    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(base) => Some(vec![(base, PossibleScore::Likely(100)).into()]),
            Err(_) => None,
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

pub(crate) fn convert_time_to_secs(input: &str, unit: TimeUnit) -> AResult<i32> {
    match unit {
        TimeUnit::Minute => {
            let time: Vec<&str> = input.trim_start_matches(['+', '-']).split(":").collect();

            Ok(time[0].parse::<i32>()? * 3600 + time[1].parse::<i32>()? * 60)
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::krate::toent::{EventBuilder, timeevent::timeenum::base::BaseTime};

    fn n(n: i32) -> Option<i32> {
        Some(n)
    }

    fn compare(guess: &str, time: BaseTime) {
        println!("guess: {}", guess);
        let c: Vec<crate::krate::toent::dto::GuessElem<BaseTime>> =
            BaseTime::guess(&guess.into()).unwrap();
        println!("guessed: {:?}", c);
        assert!(c.first().unwrap().toent == time);
    }

    #[test]
    fn test_all() {
        let ymd = BaseTime::base_time2(n(2020), n(12), n(3), None, None, None);
        let ymdh = BaseTime {
            hour: 12.into(),
            ..ymd.clone()
        };
        let ymdhm = BaseTime {
            minute: 12.into(),
            ..ymdh.clone()
        };
        let ymdhms = BaseTime {
            second: 12.into(),
            ..ymdhm.clone()
        };
        compare("2020-12-03", ymd);
        compare("2020-12-03 12", ymdh);
        compare("2020-12-03 12:12", ymdhm);
        compare("2020-12-03 12:12:12", ymdhms);

        // TODO
        // - mm-dd
        // - hh:mm
        // - hh:mm:ss
    }
}
