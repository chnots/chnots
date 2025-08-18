use chin_tools::AResult;
use serde::Serialize;
use timeenum::{Timestamp, westen::WesTime};

use crate::krate::toent::{self, dto::GuessElem};

use super::PossibleScore;

use self::{repeater::Repeater, timeenum::TimeEnum};

use super::{EventBuilder, Words};

pub(crate) mod repeater;
pub(crate) mod timeenum;

fn starts_any(input: &str, anys: &[&str]) -> bool {
    anys.iter()
        .any(|e| input.starts_with(e.to_ascii_lowercase().as_str()))
}

fn equals_any(input: &str, anys: &[&str]) -> bool {
    anys.contains(&input.to_ascii_lowercase().as_str())
}

fn contains_any(input: &str, anys: &[&str]) -> bool {
    let input = input.to_lowercase();
    anys.iter()
        .any(|e| input.contains(e.to_ascii_lowercase().as_str()))
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TimeEvent {
    base: Option<TimeEnum>,
    reminder: Option<Repeater>,
}

impl From<TimeEnum> for TimeEvent {
    fn from(value: TimeEnum) -> Self {
        Self {
            base: Some(value),
            reminder: None,
        }
    }
}

impl TimeEvent {
    pub(crate) fn now() -> Self {
        TimeEvent {
            base: Some(TimeEnum::Wes(WesTime::now_time())),
            reminder: None,
        }
    }
}

impl Serialize for TimeEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.standard_string().serialize(serializer)
    }
}

#[derive(Default, Debug)]
pub(crate) struct TimeEventSegs<'a> {
    pub(crate) base: Words<'a>,
    pub(crate) interval: Option<Words<'a>>,
    pub(crate) alert: Option<Words<'a>>,
    pub(crate) end: Option<Words<'a>>,
}

impl<'a> TryFrom<&'a Words<'a>> for TimeEventSegs<'a> {
    type Error = anyhow::Error;

    fn try_from(gt: &'a Words<'a>) -> Result<Self, Self::Error> {
        let mut input_segs = TimeEventSegs::default();

        enum ParseStep {
            Base,
            Interval,
            Alert,
            End,
        }
        let mut parse_step = ParseStep::Base;
        let mut temp: Vec<toent::Word<'_>> = vec![];

        let convert_temp = |temp: Vec<toent::Word<'a>>| {
            if !temp.is_empty() {
                let gt = Words {
                    original: gt.original,
                    words: temp,
                };
                Some(gt)
            } else {
                None
            }
        };

        for span in &gt.words {
            let seg = span.text;
            if Repeater::alter_start(seg) {
                input_segs.alert.replace(Words {
                    original: seg,
                    words: vec![*span],
                });
            } else if Repeater::interval_start(seg) {
                input_segs.interval.replace(Words {
                    original: seg,
                    words: vec![*span],
                });
            } else if Repeater::end_start(seg) {
                parse_step = ParseStep::End;
                input_segs.base = convert_temp(temp).unwrap_or_default();
                temp = vec![];
                temp.push(*span);
            } else {
                temp.push(*span);
            }
        }

        match parse_step {
            ParseStep::Base => {
                input_segs.base = convert_temp(temp).unwrap_or_default();
            }

            ParseStep::End => {
                input_segs.end = convert_temp(temp);
            }
            _ => {}
        }

        Ok(input_segs)
    }
}

impl EventBuilder for TimeEvent {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let Ok(input_segs) = TimeEventSegs::try_from(gt) else {
            return None;
        };

        let bases = TimeEnum::guess(&input_segs.base);

        let repeaters = Repeater::guess_from_segs(
            input_segs.interval.as_ref(),
            input_segs.end.as_ref(),
            input_segs.alert.as_ref(),
        );

        if let Some(time_enums) = bases {
            let mut guesses: Vec<GuessElem<TimeEvent>> = time_enums
                .iter()
                .map(|GuessElem { toent: base, score }| {
                    (
                        Self {
                            base: Some(base.clone()),
                            reminder: repeaters.first().map(|guess_elem| guess_elem.toent.clone()),
                        },
                        *score,
                    )
                        .into()
                })
                .collect();
            guesses.sort_by(|g1, g2| {
                g1.score
                    .partial_cmp(&g2.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            Some(guesses)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.base.as_ref().is_none_or(|e| e.is_valid())
            && self.reminder.as_ref().is_none_or(|e| e.is_valid())
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let input_segs = TimeEventSegs::try_from(gt)?;

        let base = Some(TimeEnum::try_from_standard(&input_segs.base)?);

        let reminder = Some(Repeater::standard_from_segs(
            input_segs.interval.as_ref(),
            input_segs.end.as_ref(),
            input_segs.alert.as_ref(),
        )?);

        Ok(TimeEvent { base, reminder })
    }

    fn standard_string(&self) -> String {
        let mut res = String::new();

        if let Some(base) = self.base.as_ref() {
            res.push_str(base.standard_string().as_str());
        }

        if let Some(rep) = &self.reminder {
            res.push(' ');
            res.push_str(rep.standard_str().as_str());
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::krate::toent::logic::{EventBuilder, timeevent::TimeEvent};

    fn guess_and_standard(guess: &str, standard: &str) {
        let guesses = TimeEvent::guess(&guess.into()).unwrap();

        let guess = guesses.first().cloned().map(|e| e.toent).unwrap();
        let standard = TimeEvent::try_from_standard(&standard.into()).unwrap();
        println!("===================");
        assert_eq!(guess, standard)
    }

    #[test]
    fn test() {
        guess_and_standard(
            "2025-12-26 12:00:00 +8:00 =2025-12-27 12:00:00",
            "2025-12-26 12:00:00 +8:00 =2025-12-27 12:00:00",
        );
    }
}
