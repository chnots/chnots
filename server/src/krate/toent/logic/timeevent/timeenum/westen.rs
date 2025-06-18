use std::ops::Deref;

use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
use regex::Regex;

use super::PossibleScore;
use super::{
    base::{convert_time_to_secs, BaseTime},
    Timestamp,
};
use crate::krate::toent::{timeevent::equals_any, EventBuilder, RawInputSegs};

pub(crate) const CAL_TYPE: &str = "wes";

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WesTime {
    offset: Option<FixedOffset>,
    timestamp: BaseTime,
}

impl Deref for WesTime {
    type Target = BaseTime;

    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}

impl From<BaseTime> for WesTime {
    fn from(value: BaseTime) -> Self {
        WesTime {
            offset: None,
            timestamp: value,
        }
    }
}

impl EventBuilder for WesTime {
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let mut guessed = vec![];
        let trimmed = gt.original;
        if trimmed.is_empty() {
            guessed.push((WesTime::now_date(), PossibleScore::Likely(100)));
        }

        if equals_any(
            trimmed.to_ascii_lowercase().as_str(),
            &["t", "n", "no", "now", "time", "uijm", "shijian", "时间"],
        ) {
            guessed.push((WesTime::now_time(), PossibleScore::Likely(100)));
        }

        if let Ok(standard) = Self::from_standard(gt) {
            guessed.push((standard, PossibleScore::Yes(100)));
        }

        Some(guessed)
    }

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let standard = &gt.spans;
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
            let sub = RawInputSegs {
                original: gt.original,
                spans: ts_segs,
            };
            let timestamp = BaseTime::from_standard(&sub)?;

            Ok(WesTime { offset, timestamp })
        }
    }

    fn standard_str(&self) -> String {
        let mut base = self.timestamp.standard_str();

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
    fn to_utc_timestamp(&self) -> DateTime<Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        CAL_TYPE
    }

    fn now_time() -> Self {
        let time = Local::now().naive_local();
        WesTime {
            offset: Default::default(),
            timestamp: BaseTime {
                year: time.year().into(),
                month: time.month().into(),
                day: time.day().into(),
                hour: time.hour().into(),
                minute: time.minute().into(),
                second: time.second().into(),
            },
        }
    }

    fn now_date() -> Self {
        let time = Local::now().naive_local();
        WesTime {
            offset: Default::default(),
            timestamp: BaseTime {
                year: time.year().into(),
                month: time.month().into(),
                day: time.day().into(),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod test {

    use crate::krate::toent::EventBuilder;

    use super::WesTime;

    #[test]
    fn from_test() {
        let wes = WesTime::from_standard(&"2020-12-02 11:12:13 +1:00".into());
        print!("{:?}", wes.unwrap().standard_str())
    }
}
