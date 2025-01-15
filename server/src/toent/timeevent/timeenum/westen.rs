use std::ops::Deref;

use chrono::{DateTime, Datelike, FixedOffset, Timelike, Utc};
use regex::Regex;

use crate::{model::score::PossibleScore, toent::{timeevent::equals_any, EventBuilder, GuessType}};

use super::{
    base::{convert_time_to_secs, BaseTime},
    Timestamp, TimestampNow,
};

pub const CAL_TYPE: &str = "wes";

#[derive(Clone, Debug)]
pub struct WesTime {
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

impl TimestampNow for WesTime {
    fn now_time() -> Self {
        let time = Utc::now().naive_local();
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
        let time = Utc::now().naive_local();
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

impl EventBuilder for WesTime {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let mut guessed = vec![];
        let trimmed = input.original;

        if equals_any(
            &trimmed.to_ascii_lowercase().as_str(),
            &["t", "now", "time", "uijm", "shijian", "时间"],
        ) {
            guessed.push((WesTime::now_time(), PossibleScore::Likely(100)));
        }

        if let Ok(standard) = Self::from_standard(&input.segs.as_slice()) {
            guessed.push((standard, PossibleScore::Yes(100)));
        }

        guessed
    }

    fn from_standard(standard: &[&str]) -> anyhow::Result<Self> {
        if standard.len() != 2 && standard.len() != 1 && standard.len() != 3 {
            anyhow::bail!("unable to parse westen timestamp: {:?}", standard)
        } else {
            let num_start: regex::Regex = Regex::new(r"^\d.*").unwrap();

            let mut ts_segs: Vec<&str> = vec![];
            let mut offset_seg = None;
            standard.into_iter().for_each(|e| {
                if num_start.is_match(e) {
                    ts_segs.push(e);
                } else {
                    offset_seg.replace(e);
                }
            });
            let offset = if let Some(o) = offset_seg {
                let value = convert_time_to_secs(o, super::base::TimeUnit::Minute)?;
                if o.starts_with("-") {
                    FixedOffset::west_opt(value)
                } else if o.starts_with("+") {
                    FixedOffset::east_opt(value)
                } else {
                    anyhow::bail!("Time offset should starts with + or -.");
                }
            } else {
                None
            };
            let timestamp = BaseTime::from_standard(ts_segs.as_slice())?;

            Ok(WesTime { offset, timestamp })
        }
    }

    fn standard_str(&self) -> String {
        let mut base = self.timestamp.standard_str();

        if let Some(offset) = self.offset {
            base.push_str(" ");
            base.push_str(&offset.to_string());
        }

        base
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

impl Timestamp for WesTime {
    fn to_wes_timestamp(&self) -> DateTime<Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        &CAL_TYPE
    }
}

#[cfg(test)]
mod test {


    use crate::toent::EventBuilder;

    use super::WesTime;

    #[test]
    fn from_test() {
        let wes = WesTime::from_standard(&["2020-12-02", "11:12:13", "+1:00"]);
        print!("{:?}", wes.unwrap().standard_str())
    }
}
