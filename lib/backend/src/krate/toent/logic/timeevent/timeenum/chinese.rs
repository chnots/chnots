use chin_tools::AResult;
use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{DateTime, Local, Timelike};
use num_traits::ToPrimitive;

use super::Timestamp;
use crate::krate::toent::dto::GuessElem;
use crate::krate::toent::timeevent::timeenum::base::{BaseDate, BaseDateTime, BaseTime};
use crate::krate::toent::{EventBuilder, Words, timeevent::contains_any};

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct ChnTime {
    leap_month: bool,
    timestamp: BaseDateTime,
}

impl Timestamp for ChnTime {
    fn to_utc_timestamp(&self) -> chrono::prelude::DateTime<chrono::prelude::Utc> {
        todo!()
    }

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
                hour: now.hour().into(),
                minute: now.minute().into(),
                second: now.second().into(),
            },
        };
        Self {
            leap_month: date.to_lunar_month().to_u8_raw() > 100,
            timestamp: ts,
        }
    }

    fn now_date() -> Self {
        let now = Self::now_time();
        Self {
            leap_month: now.leap_month,
            timestamp: BaseDateTime {
                time: BaseTime {
                    hour: Default::default(),
                    minute: Default::default(),
                    second: Default::default(),
                },
                ..now.timestamp
            },
        }
    }
}

impl ChnTime {
    pub(crate) fn new(leap_month: bool, timestamp: BaseDateTime) -> Self {
        Self {
            leap_month,
            timestamp,
        }
    }
}

impl EventBuilder for ChnTime {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let mut base_score: u8 = 0;
        let mut leap_month = false;
        if gt.full_contains_ig_case(&["农", "nong", "ns"]) {
            base_score = 128
        } else if gt.full_contains_ig_case(&["ns"]) {
            base_score = 64
        };

        if gt.full_contains_ig_case(&["闰", "run"]) {
            leap_month = true;
        }

        let bases = BaseDateTime::guess(
            &gt.filter(|e| !contains_any(e, &["闰", "run", "ns", "农", "nong", "ns"])),
        );

        if let Some(bases) = bases {
            let v = bases
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
                .collect();
            Some(v)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.timestamp.is_valid()
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let segs = &gt.words;
        let mut leap_month = false;
        if segs.len() < 2 {
            anyhow::bail!(
                "There are at least two segs for Chinese Calendar deserializition, {:?}",
                segs
            )
        } else if segs[0].text != "农" {
            anyhow::bail!("The segs do not start with 农: {:?}", segs);
        } else {
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
        EventBuilder,
        logic::timeevent::timeenum::base::BaseTime,
        timeevent::timeenum::{base::BaseDateTime, chinese::ChnTime},
    };

    #[test]
    fn test() {
        let r = ChnTime::guess(&"农 2023-12-12".into()).unwrap();
        let chn = ChnTime {
            leap_month: false,
            timestamp: BaseDateTime::default()
                .with_year(2023)
                .with_month(12)
                .with_day(12),
        };
        println!("guessed: {:?}", r);
        assert!(r.first().unwrap().timestamp == chn);
    }
}
