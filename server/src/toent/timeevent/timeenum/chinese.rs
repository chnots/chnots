use crate::{
    model::score::PossibleScore,
    toent::{timeevent::contains_any, EventBuilder, GuessType},
};

use super::{base::BaseTime, Timestamp};

#[derive(Clone, Debug)]
pub struct ChnTime {
    leap_month: bool,
    timestamp: BaseTime,
}

impl Timestamp for ChnTime {
    fn to_wes_timestamp(&self) -> chrono::prelude::DateTime<chrono::prelude::Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        todo!()
    }
}

impl EventBuilder for ChnTime {
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        let mut base_score: u8 = 0;
        let mut leap_month = false;
        if input.full_contains_ig_case(&["农", "nong", "ns"]) {
            base_score = 128
        } else if input.full_contains_ig_case(&["ns"]) {
            base_score = 64
        };

        if input.full_contains_ig_case(&["闰", "run"]) {
            leap_month = true;
        }

        let bases = BaseTime::guess(
            &input.filter(|e| !contains_any(e, &["闰", "run", "ns", "农", "nong", "ns"])),
        );

        bases
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
            .collect()
    }

    fn is_valid(&self) -> bool {
        self.timestamp.is_valid()
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        let mut leap_month = false;
        if segs.len() < 2 {
            anyhow::bail!(
                "There are at least two segs for Chinese Calendar deserializition, {:?}",
                segs
            )
        } else if segs[0] != "农" {
            anyhow::bail!("The segs do not start with 农: {:?}", segs);
        } else {
            if segs[1] == "[闰]" {
                leap_month = true;
            }

            let start = if leap_month { 2 } else { 1 };
            let timestamp = BaseTime::from_standard(&segs[start..])?;

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
        let r = ChnTime::from_standard(&["农", "2023-12-02"]);
        println!("{:?}", r);
    }
}
