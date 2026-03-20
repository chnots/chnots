use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use anyhow::{anyhow, bail};
use chin_sql::time_type::TID;
use chin_tools::AResult;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use log::info;
use serde::{Deserialize, Serialize};

use crate::krate::toent::{
    EventBuilder, Words,
    dto::GuessElem,
    timeevent::timeenum::{UtcWithOffset, UtcWithOffsetType},
};

use super::PossibleScore;
use crate::all_none;
use crate::all_some;

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
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

impl Debug for NoneOrI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ni32:")?;
        match self.0 {
            Some(c) => f.write_fmt(format_args!("{}", c)),
            None => f.write_str("nil"),
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
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct Dymd {
    pub(crate) year: NoneOrI32,
    pub(crate) month: NoneOrI32,
    pub(crate) day: NoneOrI32,
}

impl Dymd {
    pub fn all_some(&self) -> bool {
        self.year.is_some() && self.month.is_some() && self.day.is_some()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct DHMS {
    pub(crate) hour: NoneOrI32,
    pub(crate) minute: NoneOrI32,
    pub(crate) second: NoneOrI32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct DymdHMS {
    pub date: Dymd,
    pub time: DHMS,
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

impl DymdHMS {
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
            date: Dymd {
                year: y.into(),
                month: m.into(),
                day: d.into(),
            },
            time: DHMS {
                hour: hour.into(),
                minute: minute.into(),
                second: sec.into(),
            },
        }
    }

    pub fn is_seq_some(&self) -> bool {
        let date = self.date;
        let time = self.time;
        (all_some!(date.year)
            && all_none!(date.month, date.day, time.hour, time.minute, time.second))
            || (all_some!(date.year, date.month)
                && all_none!(date.day, time.hour, time.minute, time.second))
            || (all_some!(date.year, date.month, date.day)
                && all_none!(time.hour, time.minute, time.second))
            || (all_some!(date.year, date.month, date.day, time.hour)
                && all_none!(time.minute, time.second))
            || all_some!(date.year, date.month, date.day, time.hour, time.minute)
    }

    pub fn to_utc_timestamp(&self, local_minus_utc: i32) -> AResult<UtcWithOffsetType> {
        if !self.is_seq_some() {
            bail!("BaseDateTime sequence is invalid ");
        }
        let y = self.date.year.0.ok_or_else(|| anyhow!("Year missing"))?;
        let m = self.date.month.unwrap_or(1) as u32;
        let d = self.date.day.unwrap_or(1) as u32;
        let hh = self.time.hour.unwrap_or(0) as u32;
        let mm = self.time.minute.unwrap_or(0) as u32;
        let ss = self.time.second.unwrap_or(0) as u32;
        let start_nd = NaiveDate::from_ymd_opt(y, m, d).ok_or_else(|| anyhow!("Invalid Date"))?;
        let start_nt =
            NaiveTime::from_hms_opt(hh, mm, ss).ok_or_else(|| anyhow!("Invalid Time"))?;
        let start_ndt = start_nd.and_time(start_nt).and_utc();
        let to_utc_obj = |ndt: DateTime<Utc>| -> AResult<UtcWithOffset> {
            let ts = ndt.timestamp() - (local_minus_utc as i64);
            Ok(UtcWithOffset {
                utc: (ts * 1_000_000).try_into()?,
                local_minus_utc,
            })
        };
        if self.time.second.is_some() {
            Ok(UtcWithOffsetType::Point(to_utc_obj(start_ndt)?))
        } else {
            let end_ndt = if self.time.minute.is_some() {
                start_ndt + Duration::minutes(1)
            } else if self.time.hour.is_some() {
                start_ndt + Duration::hours(1)
            } else if self.date.day.is_some() {
                start_ndt + Duration::days(1)
            } else if self.date.month.is_some() {
                let (next_y, next_m) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
                let d = NaiveDate::from_ymd_opt(next_y, next_m, 1).unwrap();
                d.and_hms_opt(0, 0, 0).unwrap().and_utc()
            } else {
                let d = NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap();
                d.and_hms_opt(0, 0, 0).unwrap().and_utc()
            };
            Ok(UtcWithOffsetType::Period {
                start: to_utc_obj(start_ndt)?,
                end: to_utc_obj(end_ndt - Duration::seconds(1))?,
            })
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
    ) -> DymdHMS {
        DymdHMS {
            date: Dymd {
                year: year.into(),
                month: month.into(),
                day: day.into(),
            },
            time: DHMS {
                hour: hour.into(),
                minute: minute.into(),
                second: second.into(),
            },
        }
    }
}

#[macro_export]
macro_rules! all_some {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_some() && all_some!($($tail),*)
    };
}

#[macro_export]
macro_rules! all_none {
    () => {
        true
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head.is_none() && all_none!($($tail),*)
    };
}

impl Dymd {
    fn is_none(&self) -> bool {
        self.day.is_none() && self.month.is_none() && self.year.is_none()
    }
}

impl EventBuilder for Dymd {
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

        let bts = Dymd {
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

impl DHMS {
    fn is_none(&self) -> bool {
        self.second.is_none() && self.minute.is_none() && self.hour.is_none()
    }
}

impl EventBuilder for DHMS {
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

            let bts = DHMS {
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

impl EventBuilder for DymdHMS {
    fn is_valid(&self) -> bool {
        self.date.is_valid() && (self.time.is_valid() || self.time.is_none())
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        Ok(Self {
            date: match gt.sub1(0) {
                Some(r) => Dymd::try_from_standard(&r)?,
                None => Dymd {
                    year: Default::default(),
                    month: Default::default(),
                    day: Default::default(),
                },
            },
            time: match gt.sub1(1) {
                Some(r) => DHMS::try_from_standard(&r)?,
                None => DHMS {
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

impl From<NaiveDateTime> for DymdHMS {
    fn from(value: NaiveDateTime) -> Self {
        let c = DymdHMS {
            date: Dymd {
                year: value.year().into(),
                month: value.month().into(),
                day: value.day().into(),
            },
            time: DHMS {
                hour: value.hour().into(),
                minute: value.minute().into(),
                second: value.second().into(),
            },
        };

        c
    }
}

impl From<NaiveDate> for DymdHMS {
    fn from(value: NaiveDate) -> Self {
        DymdHMS {
            date: Dymd {
                year: value.year().into(),
                month: value.month().into(),
                day: value.day().into(),
            },
            ..Default::default()
        }
    }
}

impl From<NaiveTime> for DymdHMS {
    fn from(value: NaiveTime) -> Self {
        DymdHMS {
            time: DHMS {
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
        timeevent::timeenum::base::{DHMS, DymdHMS},
    };

    fn n(n: i32) -> Option<i32> {
        Some(n)
    }

    fn compare(guess: &str, time: DymdHMS) {
        println!("guess: {}", guess);
        let c: Vec<crate::krate::toent::dto::GuessElem<DymdHMS>> =
            DymdHMS::guess(&guess.into()).unwrap();
        println!("guessed: {:?}", c);
        assert!(c.first().unwrap().timestamp == time);
    }

    #[test]
    fn test_all() {
        let ymd = DymdHMS::base_time2(n(2020), n(12), n(3), None, None, None);
        let ymdh = DymdHMS {
            date: ymd.date,
            time: DHMS {
                hour: 12.into(),
                ..ymd.time
            },
        };
        let ymdhm = DymdHMS {
            date: ymdh.date,
            time: DHMS {
                minute: 12.into(),
                ..ymdh.time
            },
        };
        let ymdhms = DymdHMS {
            date: ymdhm.date,
            time: DHMS {
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

#[cfg(test)]
mod tests {
    use super::*;
    fn create_datetime(
        y: Option<i32>,
        m: Option<i32>,
        d: Option<i32>,
        hh: Option<i32>,
        mm: Option<i32>,
        ss: Option<i32>,
    ) -> DymdHMS {
        DymdHMS {
            date: Dymd {
                year: NoneOrI32(y),
                month: NoneOrI32(m),
                day: NoneOrI32(d),
            },
            time: DHMS {
                hour: NoneOrI32(hh),
                minute: NoneOrI32(mm),
                second: NoneOrI32(ss),
            },
        }
    }

    const OFFSET_P8: i32 = 28800;
    #[test]
    fn test_precision_year() {
        let dt = create_datetime(Some(2023), None, None, None, None, None);
        let res = dt.to_utc_timestamp(OFFSET_P8).unwrap();

        if let UtcWithOffsetType::Period { start, end } = res {
            assert_eq!(start.utc.as_num(), 167250240_0000000);
            assert_eq!(end.utc.as_num(), (1704038400 - 1) * 1000000);
        } else {
            panic!("Should be a Period");
        }
    }
    #[test]
    fn test_precision_month() {
        let dt = create_datetime(Some(2023), Some(2), None, None, None, None);
        let res = dt.to_utc_timestamp(OFFSET_P8).unwrap();

        if let UtcWithOffsetType::Period { start, end } = res {
            assert_eq!(start.utc.as_num(), 1675180800_000000);
            assert_eq!(end.utc.as_num(), 1677599999000000);
        } else {
            panic!("Should be a Period");
        }
    }
    #[test]
    fn test_precision_day() {
        let dt = create_datetime(Some(2023), Some(10), Some(27), None, None, None);
        let res = dt.to_utc_timestamp(OFFSET_P8).unwrap();

        if let UtcWithOffsetType::Period { start, end } = res {
            assert_eq!(start.utc.as_num(), 1698336000_000000);
            assert_eq!(end.utc.as_num(), 1698422399_000000);
        } else {
            panic!("Should be a Period");
        }
    }
    #[test]
    fn test_precision_minute() {
        let dt = create_datetime(Some(2023), Some(10), Some(27), Some(10), Some(30), None);
        let res = dt.to_utc_timestamp(OFFSET_P8).unwrap();

        if let UtcWithOffsetType::Period { start, end } = res {
            assert_eq!(start.utc.as_num(), 1698373800_000000);
            assert_eq!(end.utc.as_num(), 1698373859_000000);
        } else {
            panic!("Should be a Period");
        }
    }
    #[test]
    fn test_precision_second_point() {
        let dt = create_datetime(Some(2023), Some(10), Some(27), Some(10), Some(30), Some(45));
        let res = dt.to_utc_timestamp(OFFSET_P8).unwrap();

        match res {
            UtcWithOffsetType::Point(pt) => {
                assert_eq!(pt.utc.as_num(), 1698373845_000000);
                assert_eq!(pt.local_minus_utc, OFFSET_P8);
            }
            _ => panic!("Should be a Point"),
        }
    }
    #[test]
    fn test_invalid_sequence() {
        let dt = create_datetime(Some(2023), None, Some(27), None, None, None);
        let res = dt.to_utc_timestamp(OFFSET_P8);
        assert!(res.is_err());
    }
    #[test]
    fn test_leap_year_rollover() {
        let dt = create_datetime(Some(2024), Some(2), None, None, None, None);
        let res = dt.to_utc_timestamp(0).unwrap();
        if let UtcWithOffsetType::Period { start, end } = res {
            let diff = end.utc.as_num() - start.utc.as_num();
            assert_eq!(diff, (29 * 24 * 3600 - 1) * 1000000);
        }
    }
    #[test]
    fn test_year_end_rollover() {
        let dt = create_datetime(Some(2023), Some(12), None, None, None, None);
        let res = dt.to_utc_timestamp(0).unwrap();

        if let UtcWithOffsetType::Period { start, end } = res {
            assert_eq!(start.utc.as_num(), 1701388800000000);
            assert_eq!(end.utc.as_num(), 1704067199000000);
        }
    }
}
