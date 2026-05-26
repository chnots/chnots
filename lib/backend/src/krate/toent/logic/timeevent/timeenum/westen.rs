use std::ops::{Add, Deref};

use chin_tools::AResult;
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, Offset, Timelike};
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::PossibleScore;
use super::{
    Timestamp,
    base::{DymdHMS, convert_time_to_secs},
};
use crate::krate::toent::dto::GuessElem;
use crate::krate::toent::timeevent::repeater::interval::TimeInterval;
use crate::krate::toent::timeevent::timeenum::base::{DHMS, Dymd};
use crate::krate::toent::timeevent::timeenum::{UtcWithOffset, naive_datetime_add_interval};
use crate::krate::toent::{EventBuilder, Words, timeevent::equals_any};

pub(crate) const CAL_TYPE: &str = "wes";

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct WesTime {
    pub(crate) local_minus_utc: Option<i32>, // local_minus_utc
    pub(crate) timestamp: DymdHMS,
}

impl Deref for WesTime {
    type Target = DymdHMS;

    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}

impl From<DymdHMS> for WesTime {
    fn from(value: DymdHMS) -> Self {
        WesTime {
            local_minus_utc: None,
            timestamp: value,
        }
    }
}

impl From<UtcWithOffset> for WesTime {
    fn from(value: UtcWithOffset) -> Self {
        let utc_dt = DateTime::from_timestamp_micros(value.utc.as_num()).unwrap();
        let dt = utc_dt + Duration::seconds(i64::from(value.local_minus_utc));
        WesTime {
            local_minus_utc: value.local_minus_utc.into(),
            timestamp: DymdHMS {
                date: Dymd {
                    year: dt.year().into(),
                    month: dt.month().into(),
                    day: dt.day().into(),
                },
                time: DHMS {
                    hour: dt.hour().into(),
                    minute: dt.minute().into(),
                    second: dt.second().into(),
                },
            },
        }
    }
}

impl EventBuilder for WesTime {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let mut guessed = vec![];
        let trimmed = gt.original;
        if trimmed.is_empty() {
            guessed.push((WesTime::now_date(), PossibleScore::Likely(100)).into());
        }

        if equals_any(
            trimmed.to_ascii_lowercase().as_str(),
            &["t", "n", "no", "now", "time", "uijm", "shijian", "时间"],
        ) {
            guessed.push((WesTime::now_time(), PossibleScore::Likely(100)).into());
        }

        if let Ok(standard) = Self::try_from_standard(gt) {
            guessed.push((standard, PossibleScore::Yes(100)).into());
        }

        Some(guessed)
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let standard = &gt.words;
        if standard.len() != 2 && standard.len() != 1 && standard.len() != 3 {
            anyhow::bail!("unable to parse westen timestamp: {:?}", standard)
        } else {
            let num_start: regex::Regex = Regex::new(r"^\d.*").unwrap();

            let mut ts_segs = vec![];
            let mut offset_seg = None;
            standard.iter().for_each(|e| {
                if num_start.is_match(e.text) {
                    ts_segs.push(*e);
                } else {
                    offset_seg.replace(e);
                }
            });
            let offset = if let Some(span) = offset_seg {
                let value = convert_time_to_secs(span, super::base::TimeUnit::Minute)?;
                if span.starts_with("-") {
                    FixedOffset::west_opt(value)
                } else if span.starts_with("+") {
                    FixedOffset::east_opt(value)
                } else {
                    anyhow::bail!("Time offset should starts with + or -.");
                }
            } else {
                None
            };
            let sub = Words {
                original: gt.original,
                words: ts_segs,
            };
            let timestamp = DymdHMS::try_from_standard(&sub)?;

            Ok(WesTime {
                local_minus_utc: offset.map(|e| e.local_minus_utc()),
                timestamp,
            })
        }
    }

    fn standard_string(&self) -> String {
        let mut base = self.timestamp.standard_string();

        if let Some(offset) = self.local_minus_utc {
            base.push(' ');
            base.push_str(&offset.to_string());
        }

        base
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

impl Timestamp for WesTime {
    fn calender_type(&self) -> &'static str {
        CAL_TYPE
    }

    fn now_time() -> Self {
        let time = Local::now().naive_local();
        WesTime {
            local_minus_utc: Default::default(),
            timestamp: DymdHMS {
                date: Dymd {
                    year: time.year().into(),
                    month: time.month().into(),
                    day: time.day().into(),
                },
                time: DHMS {
                    hour: time.hour().into(),
                    minute: time.minute().into(),
                    second: time.second().into(),
                },
            },
        }
    }

    fn now_date() -> Self {
        let time = Local::now().naive_local();
        WesTime {
            local_minus_utc: Default::default(),
            timestamp: DymdHMS {
                date: Dymd {
                    year: time.year().into(),
                    month: time.month().into(),
                    day: time.day().into(),
                },
                ..Default::default()
            },
        }
    }

    fn to_utc_timestamp(&self) -> AResult<super::UtcWithOffsetType> {
        self.timestamp.to_utc_timestamp(
            self.local_minus_utc
                .unwrap_or(Local::now().fixed_offset().offset().fix().local_minus_utc()),
        )
    }
}

impl Add<TimeInterval> for WesTime {
    type Output = Option<WesTime>;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        let start = match self.to_utc_timestamp() {
            Ok(v) => v.start(),
            Err(_) => return None,
        };
        let ndt = DateTime::from_timestamp_micros(start.utc().as_num())
            .map(|e| e.naive_utc() + Duration::seconds(i64::from(start.local_minus_utc())));

        let ndt = naive_datetime_add_interval(ndt, rhs);

        ndt.map(|ndt| WesTime {
            local_minus_utc: self.local_minus_utc,
            timestamp: ndt.into(),
        })
    }
}

#[cfg(test)]
mod test {

    use chrono::{FixedOffset, NaiveDate, NaiveTime};

    use crate::krate::toent::{
        EventBuilder,
        logic::timeevent::timeenum::base::DymdHMS,
        timeevent::{
            repeater::interval::TimeInterval,
            timeenum::{
                base::{DHMS, Dymd},
                naive_datetime_add_interval,
            },
        },
    };

    use super::WesTime;

    #[test]
    fn add_test() {
        let s: Option<chrono::NaiveDateTime> = naive_datetime_add_interval(
            NaiveDate::from_ymd_opt(2026, 03, 17)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap())
                .into(),
            TimeInterval {
                base: DymdHMS {
                    date: Dymd {
                        year: 0.into(),
                        month: 0.into(),
                        day: 1.into(),
                    },
                    time: DHMS {
                        hour: 0.into(),
                        minute: 0.into(),
                        second: 0.into(),
                    },
                },
                week: 0.into(),
            },
        );
        assert_eq!(
            s,
            Some(
                NaiveDate::from_ymd_opt(2026, 03, 18)
                    .unwrap()
                    .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap())
            )
        )
    }

    #[test]
    fn from_test() {
        let ymdhms = DymdHMS {
            date: Dymd {
                year: 2020.into(),
                month: 12.into(),
                day: 12.into(),
            },
            time: DHMS {
                hour: 12.into(),
                minute: 12.into(),
                second: 12.into(),
            },
        };

        let guesses = WesTime::guess(&"2020-12-12 12:12:12 +8:00".into()).unwrap();
        let guessed = guesses.first().unwrap();
        println!("{:?}", guessed);
        assert!(
            WesTime {
                local_minus_utc: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
                timestamp: ymdhms.clone()
            } == guessed.timestamp
        );

        let guesses = WesTime::guess(&"2020-12-12 12:12:12".into()).unwrap();
        let guessed = guesses.first().unwrap();
        println!("{:?}", guessed);
        assert!(
            WesTime {
                local_minus_utc: None,
                timestamp: ymdhms.clone()
            } == guessed.timestamp
        );
    }

    #[test]
    fn from_utc_with_offset_test() {
        let utc_tid = (1735689600_i64 * 1_000_000).try_into().unwrap();
        let wes: WesTime =
            crate::krate::toent::timeevent::timeenum::UtcWithOffset::new(utc_tid, 8 * 3600).into();

        assert_eq!(
            wes,
            WesTime {
                local_minus_utc: Some(8 * 3600),
                timestamp: DymdHMS::new(2025, 1, 1, 8, 0, 0),
            }
        );
    }
}
