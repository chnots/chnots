use std::{fmt::Display, ops::Deref};

use chin_tools::AResult;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

use crate::krate::toent::{EventBuilder, Words, dto::GuessElem};

use super::PossibleScore;

#[derive(Copy, Clone, Deserialize, Serialize, Default, Debug, PartialEq)]
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct BaseDate {
    pub(crate) year: NoneOrI32,
    pub(crate) month: NoneOrI32,
    pub(crate) day: NoneOrI32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct BaseTime {
    pub(crate) hour: NoneOrI32,
    pub(crate) minute: NoneOrI32,
    pub(crate) second: NoneOrI32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct BaseDateTime {
    pub date: BaseDate,
    pub time: BaseTime,
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

impl BaseDateTime {
    pub fn empty() -> Self {
        Self::new(
            None::<i32>,
            None::<i32>,
            None::<i32>,
            None::<i32>,
            None::<i32>,
            None::<i32>,
        )
    }

    pub fn new<T: Into<NoneOrI32>>(y: T, m: T, d: T, hour: T, minute: T, sec: T) -> Self {
        Self {
            date: BaseDate {
                year: y.into(),
                month: m.into(),
                day: d.into(),
            },
            time: BaseTime {
                hour: hour.into(),
                minute: minute.into(),
                second: sec.into(),
            },
        }
    }

    pub(crate) fn with_year(mut self, year: i32) -> Self {
        self.date.year = year.into();
        self
    }

    pub(crate) fn with_month(mut self, month: i32) -> Self {
        self.date.month = month.into();
        self
    }

    pub(crate) fn with_day(mut self, day: i32) -> Self {
        self.date.day = day.into();
        self
    }
    pub(crate) fn with_hour(mut self, hour: i32) -> Self {
        self.time.hour = hour.into();
        self
    }
    pub(crate) fn with_minute(mut self, minute: i32) -> Self {
        self.time.minute = minute.into();
        self
    }

    pub(crate) fn with_second(mut self, second: i32) -> Self {
        self.time.second = second.into();
        self
    }

    fn base_time2(
        year: Option<i32>,
        month: Option<i32>,
        day: Option<i32>,
        hour: Option<i32>,
        minute: Option<i32>,
        second: Option<i32>,
    ) -> BaseDateTime {
        BaseDateTime {
            date: BaseDate {
                year: year.into(),
                month: month.into(),
                day: day.into(),
            },
            time: BaseTime {
                hour: hour.into(),
                minute: minute.into(),
                second: second.into(),
            },
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

impl BaseDate {
    fn is_none(&self) -> bool {
        self.day.is_none() && self.month.is_none() && self.year.is_none()
    }
}

impl EventBuilder for BaseDate {
    fn is_valid(&self) -> bool {
        all_some!(self.year, self.month, self.day)
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let filted = gt.filterd();
        if !filted.contains("-") {
            anyhow::bail!("There should be like '2012-12-12', found {:?}", gt)
        }
        let mut year = None::<String>;
        let mut month = None::<String>;
        let mut day = None::<String>;

        for (index, value) in filted.split("-").enumerate() {
            match index {
                0 => year = Some(value.into()),
                1 => month = Some(value.into()),
                2 => day = Some(value.into()),
                _ => {}
            }
        }

        let bts = BaseDate {
            year: year.into(),
            month: month.into(),
            day: day.into(),
        };

        if bts.is_valid() {
            Ok(bts)
        } else {
            anyhow::bail!("unable to parse timestamp: {:?}", gt.original)
        }
    }

    fn standard_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }

    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(base) => Some(vec![(base, PossibleScore::Likely(100)).into()]),
            Err(_) => None,
        }
    }
}

impl BaseTime {
    fn is_none(&self) -> bool {
        self.second.is_none() && self.minute.is_none() && self.hour.is_none()
    }
}

impl EventBuilder for BaseTime {
    fn is_valid(&self) -> bool {
        all_some!(self.hour, self.minute)
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let standard = &gt.filterd();
        if !standard.contains(":") {
            anyhow::bail!("There should be like '20:00:00', found {:?}", standard)
        } else {
            let mut hour = None::<String>;
            let mut minute = None::<String>;
            let mut second = None::<String>;

            for (index, value) in standard.split(":").enumerate() {
                match index {
                    0 => hour = Some(value.into()),
                    1 => minute = Some(value.into()),
                    2 => second = Some(value.into()),
                    _ => {}
                }
            }

            let bts = BaseTime {
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
            format!("{}:{}:{}", self.hour, self.minute, self.second)
        } else {
            format!("{}:{}", self.hour, self.minute)
        }
    }

    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(base) => Some(vec![(base, PossibleScore::Likely(100)).into()]),
            Err(_) => None,
        }
    }
}

impl EventBuilder for BaseDateTime {
    fn is_valid(&self) -> bool {
        self.date.is_valid() && (self.time.is_valid() || self.time.is_none())
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        Ok(Self {
            date: match gt.sub1(0) {
                Some(r) => BaseDate::try_from_standard(&r)?,
                None => BaseDate {
                    year: Default::default(),
                    month: Default::default(),
                    day: Default::default(),
                },
            },
            time: match gt.sub1(1) {
                Some(r) => BaseTime::try_from_standard(&r)?,
                None => BaseTime {
                    hour: Default::default(),
                    minute: Default::default(),
                    second: Default::default(),
                },
            },
        })
    }

    fn standard_string(&self) -> String {
        if self.time.is_none() {
            self.date.standard_string()
        } else {
            format!(
                "{} {}",
                self.date.standard_string(),
                self.time.standard_string()
            )
        }
    }

    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(base) => Some(vec![(base, PossibleScore::Likely(100)).into()]),
            Err(_) => None,
        }
    }
}

impl From<NaiveDateTime> for BaseDateTime {
    fn from(value: NaiveDateTime) -> Self {
        BaseDateTime {
            date: BaseDate {
                year: value.year().into(),
                month: value.month().into(),
                day: value.day().into(),
            },
            time: BaseTime {
                hour: value.hour().into(),
                minute: value.minute().into(),
                second: value.second().into(),
            },
        }
    }
}

impl From<NaiveDate> for BaseDateTime {
    fn from(value: NaiveDate) -> Self {
        BaseDateTime {
            date: BaseDate {
                year: value.year().into(),
                month: value.month().into(),
                day: value.day().into(),
            },
            ..Default::default()
        }
    }
}

impl From<NaiveTime> for BaseDateTime {
    fn from(value: NaiveTime) -> Self {
        BaseDateTime {
            time: BaseTime {
                hour: value.hour().into(),
                minute: value.minute().into(),
                second: value.second().into(),
            },
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
    use crate::krate::toent::{
        EventBuilder,
        timeevent::timeenum::base::{BaseDateTime, BaseTime},
    };

    fn n(n: i32) -> Option<i32> {
        Some(n)
    }

    fn compare(guess: &str, time: BaseDateTime) {
        println!("guess: {}", guess);
        let c: Vec<crate::krate::toent::dto::GuessElem<BaseDateTime>> =
            BaseDateTime::guess(&guess.into()).unwrap();
        println!("guessed: {:?}", c);
        assert!(c.first().unwrap().timestamp == time);
    }

    #[test]
    fn test_all() {
        let ymd = BaseDateTime::base_time2(n(2020), n(12), n(3), None, None, None);
        let ymdh = BaseDateTime {
            date: ymd.date,
            time: BaseTime {
                hour: 12.into(),
                ..ymd.time
            },
        };
        let ymdhm = BaseDateTime {
            date: ymdh.date,
            time: BaseTime {
                minute: 12.into(),
                ..ymdh.time
            },
        };
        let ymdhms = BaseDateTime {
            date: ymdhm.date,
            time: BaseTime {
                second: 12.into(),
                ..ymdhm.time
            },
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
