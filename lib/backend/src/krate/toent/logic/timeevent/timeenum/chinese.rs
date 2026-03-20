use std::ops::Add;

use anyhow::{anyhow, bail};
use chin_tools::AResult;
use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, Timelike};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

pub(crate) use super::chinese_cal_calc::ChnTimeCalculator;
use super::{Timestamp, UtcWithOffset, UtcWithOffsetType};
use crate::krate::toent::dto::GuessElem;
use crate::krate::toent::timeevent::contains_any;
use crate::krate::toent::timeevent::repeater::interval::TimeInterval;
use crate::krate::toent::timeevent::timeenum::base::{BaseDate, BaseDateTime, BaseTime};
use crate::krate::toent::{EventBuilder, Words};

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct ChnTime {
    pub(crate) leap_month: bool,
    pub(crate) timestamp: BaseDateTime,
}

impl ChnTime {
    pub(crate) fn new(leap_month: bool, timestamp: BaseDateTime) -> Self {
        Self {
            leap_month,
            timestamp,
        }
    }
}

impl Add<TimeInterval> for ChnTime {
    type Output = Option<ChnTime>;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        self.add_duration(rhs).ok()
    }
}

impl From<UtcWithOffset> for ChnTime {
    fn from(value: UtcWithOffset) -> Self {
        let dt = DateTime::from_timestamp_micros(value.utc.as_num()).unwrap();
        let date = LunisolarDate::from_date(dt).unwrap();
        let ts = BaseDateTime {
            date: BaseDate {
                year: date.to_solar_year().to_i32().into(),
                month: date.to_lunar_month().to_u8().to_i32().unwrap().into(),
                day: date.to_lunar_day().to_u8().to_i32().unwrap().into(),
            },
            time: BaseTime {
                hour: dt.hour().to_i32().unwrap().into(),
                minute: dt.minute().to_i32().unwrap().into(),
                second: dt.second().to_i32().unwrap().into(),
            },
        };

        Self {
            leap_month: date.to_lunar_month().is_leap_month(),
            timestamp: ts,
        }
    }
}

impl Timestamp for ChnTime {
    fn calender_type(&self) -> &'static str {
        "chn"
    }

    fn now_time() -> Self {
        let now: DateTime<Local> = Local::now();
        let date = LunisolarDate::from_date(now).unwrap();
        let ts = BaseDateTime {
            date: BaseDate {
                year: date.to_solar_year().to_i32().into(),
                month: date.to_lunar_month().to_u8().to_i32().unwrap().into(),
                day: date.to_lunar_day().to_u8().to_i32().unwrap().into(),
            },
            time: BaseTime {
                hour: now.hour().to_i32().unwrap().into(),
                minute: now.minute().to_i32().unwrap().into(),
                second: now.second().to_i32().unwrap().into(),
            },
        };

        Self {
            leap_month: date.to_lunar_month().is_leap_month(),
            timestamp: ts,
        }
    }

    fn now_date() -> Self {
        let now = Self::now_time();
        Self {
            leap_month: now.leap_month,
            timestamp: BaseDateTime {
                time: BaseTime::default(),
                ..now.timestamp
            },
        }
    }

    fn to_utc_timestamp(&self) -> AResult<UtcWithOffsetType> {
        if !self.timestamp.is_seq_some() {
            bail!("date is not seq some {}", self.timestamp.standard_string());
        }

        let local_minus_utc = Local::now().offset().local_minus_utc();
        let (year, month, day, hour, minute, second) = self.extract_datetime_parts()?;

        let start_date =
            LunisolarDate::from_ymd(year as u16, month as u8, self.leap_month, day as u8)?;

        let start_time = NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32)
            .ok_or_else(|| anyhow!("Invalid time: {hour}:{minute}:{second}"))?;

        let start_ndt = NaiveDateTime::new(start_date.to_naive_date(), start_time);

        let to_utc_obj = |ndt: NaiveDateTime| -> AResult<UtcWithOffset> {
            let ts = ndt.and_utc().timestamp() - (local_minus_utc as i64);
            Ok(UtcWithOffset {
                utc: (ts * 1_000_000).try_into()?,
                local_minus_utc,
            })
        };

        if self.timestamp.time.second.is_some() {
            Ok(UtcWithOffsetType::Point(to_utc_obj(start_ndt)?))
        } else {
            let end_ndt = if self.timestamp.time.minute.is_some() {
                start_ndt + Duration::minutes(1)
            } else if self.timestamp.time.hour.is_some() {
                start_ndt + Duration::hours(1)
            } else if self.timestamp.date.day.is_some() {
                start_ndt + Duration::days(1)
            } else if self.timestamp.date.month.is_some() {
                self.add_duration(TimeInterval {
                    base: BaseDateTime::default().with_month(1),
                    week: Default::default(),
                })?
                .timestamp_start_naive_datetime()?
            } else {
                self.add_duration(TimeInterval {
                    base: BaseDateTime::default().with_year(1),
                    week: Default::default(),
                })?
                .timestamp_start_naive_datetime()?
            };

            Ok(UtcWithOffsetType::Period {
                start: to_utc_obj(start_ndt)?,
                end: to_utc_obj(end_ndt - Duration::seconds(1))?,
            })
        }
    }
}

impl ChnTime {
    pub(crate) fn add_duration(&self, delta: TimeInterval) -> AResult<Self> {
        ChnTimeCalculator::add(self, delta)
    }

    pub(crate) fn extract_datetime_parts(&self) -> AResult<(i32, i32, i32, i32, i32, i32)> {
        let year = self
            .timestamp
            .date
            .year
            .as_ref()
            .copied()
            .ok_or_else(|| anyhow!("Missing lunar year"))?;
        let month = self.timestamp.date.month.as_ref().copied().unwrap_or(1);
        let day = self.timestamp.date.day.as_ref().copied().unwrap_or(1);
        let hour = self.timestamp.time.hour.as_ref().copied().unwrap_or(0);
        let minute = self.timestamp.time.minute.as_ref().copied().unwrap_or(0);
        let second = self.timestamp.time.second.as_ref().copied().unwrap_or(0);

        Ok((year, month, day, hour, minute, second))
    }

    fn timestamp_start_naive_datetime(&self) -> AResult<NaiveDateTime> {
        let (year, month, day, hour, minute, second) = self.extract_datetime_parts()?;
        let start_date =
            LunisolarDate::from_ymd(year as u16, month as u8, self.leap_month, day as u8)?;
        let start_time = NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32)
            .ok_or_else(|| anyhow!("Invalid time: {hour}:{minute}:{second}"))?;

        Ok(NaiveDateTime::new(start_date.to_naive_date(), start_time))
    }
}

impl EventBuilder for ChnTime {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let mut base_score: u8 = 0;
        let mut leap_month = false;

        if gt.full_contains_ig_case(&["农", "nong", "ns"]) {
            base_score = 128;
        } else if gt.full_contains_ig_case(&["ns"]) {
            base_score = 64;
        }

        if gt.full_contains_ig_case(&["闰", "run"]) {
            leap_month = true;
        }

        let bases = BaseDateTime::guess(
            &gt.filter(|e| !contains_any(e, &["闰", "run", "ns", "农", "nong"])),
        );

        bases.map(|bases| {
            bases
                .into_iter()
                .map(
                    |GuessElem {
                         timestamp: toent,
                         score,
                     }| {
                        (
                            ChnTime {
                                leap_month,
                                timestamp: toent,
                            },
                            score.merge(base_score),
                        )
                            .into()
                    },
                )
                .collect()
        })
    }

    fn is_valid(&self) -> bool {
        self.timestamp.is_valid()
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let segs = &gt.words;
        let mut leap_month = false;

        if segs.len() < 2 {
            bail!(
                "There are at least two segs for Chinese Calendar deserializition, {:?}",
                segs
            );
        }

        if segs[0].text != "农" {
            bail!("The segs do not start with 农: {:?}", segs);
        }

        if segs[1].text == "[闰]" {
            leap_month = true;
        }

        let start = if leap_month { 2 } else { 1 };
        let timestamp = BaseDateTime::try_from_standard(&gt.sub_start(start))?;

        Ok(ChnTime {
            leap_month,
            timestamp,
        })
    }

    fn standard_string(&self) -> String {
        format!(
            "农{} {}",
            if self.leap_month { " [闰]" } else { "" },
            self.timestamp.standard_string()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::krate::toent::{
        EventBuilder, logic::timeevent::timeenum::base::BaseDateTime,
        timeevent::timeenum::chinese::ChnTime,
    };

    #[test]
    fn test_guess() {
        let r = ChnTime::guess(&"农 2023-12-12".into()).unwrap();
        let chn = ChnTime {
            leap_month: false,
            timestamp: BaseDateTime::default()
                .with_year(2023)
                .with_month(12)
                .with_day(12),
        };
        assert!(r.first().unwrap().timestamp == chn);
    }
}
