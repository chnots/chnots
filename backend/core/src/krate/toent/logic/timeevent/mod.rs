use timeenum::{Timestamp, westen::WesTime};

use crate::krate::toent;

use super::PossibleScore;

use self::{repeater::Repeater, timeenum::TimeEnum};

use super::{EventBuilder, RawInputSegs};

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

#[derive(Default)]
pub(crate) struct InputSegs<'a> {
    pub(crate) base: RawInputSegs<'a>,
    pub(crate) interval: Option<RawInputSegs<'a>>,
    pub(crate) alert: Option<RawInputSegs<'a>>,
    pub(crate) end: Option<RawInputSegs<'a>>,
}

impl<'a> TryFrom<&'a RawInputSegs<'a>> for InputSegs<'a> {
    type Error = anyhow::Error;

    fn try_from(gt: &'a RawInputSegs<'a>) -> Result<Self, Self::Error> {
        let mut input_segs = InputSegs::default();

        enum ParseStep {
            Base,
            Interval,
            Alert,
            End,
        }
        let mut parse_step = ParseStep::Base;
        let mut temp: Vec<toent::Span<'_>> = vec![];

        let convert_temp = |temp: Vec<toent::Span<'a>>| {
            if !temp.is_empty() {
                let gt = RawInputSegs {
                    original: gt.original,
                    spans: temp,
                };
                Some(gt)
            } else {
                None
            }
        };

        for span in &gt.spans {
            let seg = span.text;
            if Repeater::alter_start(seg) {
                input_segs.alert.replace(RawInputSegs {
                    original: seg,
                    spans: vec![*span],
                });
            } else if Repeater::interval_start(seg) {
                input_segs.interval.replace(RawInputSegs {
                    original: seg,
                    spans: vec![*span],
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
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        let Ok(input_segs) = InputSegs::try_from(gt) else {
            return None;
        };

        let bases = TimeEnum::guess(&input_segs.base);

        let repeaters = Repeater::guess_from_segs(
            input_segs.interval.as_ref(),
            input_segs.end.as_ref(),
            input_segs.alert.as_ref(),
        );

        if let Some(time_enums) = bases {
            let guesses = time_enums
                .iter()
                .map(|(base, score)| {
                    (
                        Self {
                            base: Some(base.clone()),
                            reminder: repeaters.first().map(|(e, _)| e.clone()),
                        },
                        *score,
                    )
                })
                .collect();
            Some(guesses)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.base.as_ref().is_none_or(|e| e.is_valid())
            && self.reminder.as_ref().is_none_or(|e| e.is_valid())
    }

    fn try_from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let input_segs = InputSegs::try_from(gt)?;

        let base = Some(TimeEnum::try_from_standard(&input_segs.base)?);

        let reminder = Some(Repeater::standard_from_segs(
            input_segs.interval.as_ref(),
            input_segs.end.as_ref(),
            input_segs.alert.as_ref(),
        )?);

        Ok(TimeEvent { base, reminder })
    }

    fn standard_str(&self) -> String {
        let mut res = String::new();

        if let Some(base) = self.base.as_ref() {
            res.push_str(base.standard_str().as_str());
        }

        if let Some(rep) = &self.reminder {
            res.push(' ');
            res.push_str(rep.standard_str().as_str());
        }

        res
    }
}
