use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{DateTime, Local, Timelike};
use num_traits::ToPrimitive;

use super::PossibleScore;
use super::{base::BaseTime, Timestamp};
use crate::mapper::db::sqlite::sqltype::Timestamptz;
use crate::toent::{timeevent::contains_any, EventBuilder, RawInputSegs};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ChnTime {
    leap_month: bool,
    timestamp: BaseTime,
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
        let ts = BaseTime {
            year: date.to_solar_year().to_i32().into(),
            month: date.to_lunar_month().to_u8().to_i32().unwrap().into(),
            day: date.to_lunar_day().to_u8().to_i32().unwrap().into(),
            hour: now.hour().into(),
            minute: now.minute().into(),
            second: now.second().into(),
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
            timestamp: BaseTime {
                hour: Default::default(),
                minute: Default::default(),
                second: Default::default(),
                ..now.timestamp
            },
        }
    }
}

impl EventBuilder for ChnTime {
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
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

        let bases = BaseTime::guess(
            &gt.filter(|e| !contains_any(e, &["闰", "run", "ns", "农", "nong", "ns"])),
        );

        if let Some(bases) = bases {
            let v = bases
                .into_iter()
                .map(|(t, score)| {
                    (
                        ChnTime {
                            leap_month,
                            timestamp: t,
                        },
                        score.merge(base_score),
                    )
                })
                .collect();
            Some(v)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.timestamp.is_valid()
    }

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let segs = &gt.spans;
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
            let timestamp = BaseTime::from_standard(&gt.sub_start(start))?;

            Ok(ChnTime {
                leap_month,
                timestamp,
            })
        }
    }

    fn standard_str(&self) -> String {
        format!(
            "农{} {}",
            if self.leap_month { " [闰]" } else { "" },
            self.timestamp.standard_str()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::toent::{timeevent::timeenum::chinese::ChnTime, EventBuilder};

    #[test]
    fn test() {
        let r = ChnTime::from_standard(&"农 2023-12-02".into());
        println!("{:?}", r);
    }
}
