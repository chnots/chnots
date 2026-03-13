use std::ops::{Add, Deref};

use chin_tools::AResult;
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, Offset, Timelike, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::PossibleScore;
use super::{
    Timestamp,
    base::{BaseDateTime, convert_time_to_secs},
};
use crate::krate::toent::dto::GuessElem;
use crate::krate::toent::timeevent::repeater::interval::TimeInterval;
use crate::krate::toent::timeevent::timeenum::base::{BaseDate, BaseTime};
use crate::krate::toent::{EventBuilder, Words, timeevent::equals_any};

pub(crate) const CAL_TYPE: &str = "wes";

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct WesTime {
    pub(crate) offset: Option<i32>, // local_minus_utc
    pub(crate) timestamp: BaseDateTime,
}

impl Deref for WesTime {
    type Target = BaseDateTime;

    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}

impl From<BaseDateTime> for WesTime {
    fn from(value: BaseDateTime) -> Self {
        WesTime {
            offset: None,
            timestamp: value,
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
            let timestamp = BaseDateTime::try_from_standard(&sub)?;

            Ok(WesTime {
                offset: offset.map(|e| e.local_minus_utc()),
                timestamp,
            })
        }
    }

    fn standard_string(&self) -> String {
        let mut base = self.timestamp.standard_string();

        if let Some(offset) = self.offset {
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
            offset: Default::default(),
            timestamp: BaseDateTime {
                date: BaseDate {
                    year: time.year().into(),
                    month: time.month().into(),
                    day: time.day().into(),
                },
                time: BaseTime {
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
            offset: Default::default(),
            timestamp: BaseDateTime {
                date: BaseDate {
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
            self.offset
                .unwrap_or(Local::now().fixed_offset().offset().fix().local_minus_utc()),
        )
    }
}

impl Add<TimeInterval> for WesTime {
    type Output = WesTime;

    fn add(self, rhs: TimeInterval) -> Self::Output {
        let start = match self.to_utc_timestamp() {
            Ok(v) => v.start(),
            Err(_) => return self,
        };

        let offset = start.local_minus_utc();
        let utc_dt = start.utc().as_utc().naive_utc();
        let local_dt = utc_dt + Duration::seconds(i64::from(offset));

        let with_ym = match add_gregorian_year_month(
            local_dt,
            rhs.date.year.unwrap_or(0),
            rhs.date.month.unwrap_or(0),
        ) {
            Some(v) => v,
            None => return self,
        };

        let day_offset =
            i64::from(rhs.date.day.unwrap_or(0)) + i64::from(rhs.week.unwrap_or(0)) * 7;
        let result = with_ym
            + Duration::days(day_offset)
            + Duration::hours(i64::from(rhs.time.hour.unwrap_or(0)))
            + Duration::minutes(i64::from(rhs.time.minute.unwrap_or(0)))
            + Duration::seconds(i64::from(rhs.time.second.unwrap_or(0)));

        WesTime {
            timestamp: result.into(),
            ..self
        }
    }
}

fn add_gregorian_year_month(
    base: chrono::NaiveDateTime,
    years: i32,
    months: i32,
) -> Option<chrono::NaiveDateTime> {
    let total_month = base.year() * 12 + (base.month() as i32 - 1) + years * 12 + months;
    let target_year = total_month.div_euclid(12);
    let target_month = total_month.rem_euclid(12) + 1;

    let mut day = base.day();
    while day >= 1 {
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(target_year, target_month as u32, day)
            && let Some(time) =
                chrono::NaiveTime::from_hms_opt(base.hour(), base.minute(), base.second())
        {
            return Some(chrono::NaiveDateTime::new(date, time));
        }
        day -= 1;
    }

    None
}

#[cfg(test)]
mod test {

    use chrono::FixedOffset;

    use crate::krate::toent::{
        EventBuilder,
        logic::timeevent::timeenum::base::BaseDateTime,
        timeevent::timeenum::base::{BaseDate, BaseTime},
    };

    use super::WesTime;

    #[test]
    fn from_test() {
        let ymdhms = BaseDateTime {
            date: BaseDate {
                year: 2020.into(),
                month: 12.into(),
                day: 12.into(),
            },
            time: BaseTime {
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
                offset: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
                timestamp: ymdhms.clone()
            } == guessed.timestamp
        );

        let guesses = WesTime::guess(&"2020-12-12 12:12:12".into()).unwrap();
        let guessed = guesses.first().unwrap();
        println!("{:?}", guessed);
        assert!(
            WesTime {
                offset: None,
                timestamp: ymdhms.clone()
            } == guessed.timestamp
        );
    }
}
