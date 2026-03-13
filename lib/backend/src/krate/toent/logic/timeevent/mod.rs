use anyhow::bail;
use chin_tools::AResult;
use serde::{Deserialize, Serialize};
use timeenum::{Timestamp, westen::WesTime};

use crate::krate::toent::{
    ToentInstCore,
    dto::GuessElem,
    timeevent::{
        repeater::{RepeatType, endconditon::EndCondition, interval::TimeInterval},
        timeenum::UtcWithOffset,
    },
};

use super::PossibleScore;

use self::timeenum::TimeEnum;

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub(crate) struct TimeEvent {
    pub start: Option<TimeEnum>,
    pub interval: Option<(TimeInterval, RepeatType)>,
    pub interval_end: Option<EndCondition>,
    pub alert: Option<TimeInterval>,
    pub end: Option<EndCondition>,
}

impl std::hash::Hash for TimeEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.interval.hash(state);
        self.interval_end.hash(state);
        self.alert.hash(state);
        self.end.hash(state);
    }
}

impl From<TimeEnum> for TimeEvent {
    fn from(value: TimeEnum) -> Self {
        Self {
            start: Some(value),
            interval: None,
            alert: None,
            end: None,
            interval_end: None,
        }
    }
}

impl TimeEvent {
    pub(crate) fn now() -> Self {
        TimeEvent {
            start: Some(TimeEnum::Wes(WesTime::now_time())),
            interval: None,
            alert: None,
            end: None,
            interval_end: None,
        }
    }

    pub(crate) fn to_insts(
        &self,
        max_count: usize,
        end_time: Option<UtcWithOffset>,
    ) -> AResult<Vec<ToentInstCore>> {
        let Some(start) = self.start else {
            bail!("start time is none");
        };

        let mut result = vec![];
        let mut point = start;
        let mut count = max_count;
        let mut end_utc = None;
        match self.end {
            Some(end_time) => match end_time {
                EndCondition::Times(times) => count = count.min(times.count() as usize),
                EndCondition::Interval(time_interval) => {
                    end_utc.replace((start + time_interval).to_utc_timestamp()?.end());
                }
                EndCondition::Time(time_enum) => {
                    end_utc.replace(time_enum.to_utc_timestamp()?.end());
                }
            },
            None => {
                end_utc = end_time;
            }
        };

        // todo get min end_time

        let is_lunar = self.start.is_some_and(|e| match e {
            TimeEnum::Wes(_wes_time) => false,
            TimeEnum::Chn(_chn_time) => true,
        });
        let timezone = if is_lunar {
            None
        } else {
            Some(start.to_utc_timestamp()?.start().local_minus_utc() as isize)
        };

        if let Some((interval, _)) = self.interval {
            for c in 0..count {
                let next = point + interval;
                let next_utc = next.to_utc_timestamp()?;
                if end_utc.is_none_or(|e| e.utc() > next_utc.start().utc()) {
                    result.push(ToentInstCore {
                        todo_state: None,
                        todo_priority: None,
                        alert_tid: None,
                        start_tid: Some(next_utc.start().utc()),
                        end_tid: None,
                        is_lunar: is_lunar,
                        timezone: timezone,
                    });
                }
                point = next;
            }
        } else {
        }

        Ok(result)
    }
}

impl EventBuilder for TimeEvent {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        let segs = split_segs(gt);

        let base_input = segs.base.unwrap_or_else(Words::empty);
        let bases = TimeEnum::guess(&base_input)?;

        let interval = parse_interval(segs.interval.as_ref()).ok()?;
        let interval_end = parse_interval_end(segs.interval_end.as_ref()).ok()?;
        let alert = parse_alert(segs.alert.as_ref()).ok()?;
        let end_cond = parse_end(segs.end.as_ref()).ok()?;
        let has_repeaters =
            interval.is_some() || interval_end.is_some() || alert.is_some() || end_cond.is_some();

        let mut guesses: Vec<GuessElem<TimeEvent>> = bases
            .into_iter()
            .map(
                |GuessElem {
                     timestamp: base,
                     score,
                 }| {
                    (
                        Self {
                            start: Some(base),
                            interval: interval.clone(),
                            interval_end: interval_end.clone(),
                            alert: alert.clone(),
                            end: end_cond.clone(),
                        },
                        if has_repeaters {
                            score.merge(96)
                        } else {
                            score
                        },
                    )
                        .into()
                },
            )
            .collect();
        guesses.sort_by(|g1, g2| {
            g2.score
                .partial_cmp(&g1.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Some(guesses)
    }

    fn is_valid(&self) -> bool {
        self.start.as_ref().is_none_or(|e| e.is_valid())
            && self.alert.as_ref().is_none_or(|e| e.is_valid())
            && self.interval.as_ref().is_none_or(|e| e.0.is_valid())
            && self.interval_end.as_ref().is_none_or(|e| e.is_valid())
            && self.end.as_ref().is_none_or(|e| e.is_valid())
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let segs = split_segs(gt);

        let base = match segs.base {
            Some(start) => Some(TimeEnum::try_from_standard(&start)?),
            None => None,
        };

        Ok(TimeEvent {
            start: base,
            interval: parse_interval(segs.interval.as_ref())?,
            interval_end: parse_interval_end(segs.interval_end.as_ref())?,
            alert: parse_alert(segs.alert.as_ref())?,
            end: parse_end(segs.end.as_ref())?,
        })
    }

    fn standard_string(&self) -> String {
        let mut parts = vec![];

        if let Some(base) = self.start.as_ref() {
            parts.push(base.standard_string());
        }

        if let Some(alert) = self.alert.as_ref() {
            parts.push(format!(",{}", alert.standard_string()));
        }

        if let Some((interval, repeat_type)) = self.interval.as_ref() {
            let marker = match repeat_type {
                RepeatType::RepeatTodo => "**",
                RepeatType::RepeatEvent => "*",
                RepeatType::Once => ".",
            };
            parts.push(format!("{marker}{}", interval.standard_string()));
        }

        if let Some(interval_end) = self.interval_end.as_ref() {
            parts.push(format!("*{}", interval_end.standard_string()));
        }

        if let Some(end_cond) = self.end.as_ref() {
            parts.push(end_cond.standard_string());
        }

        parts.join(" ")
    }
}

#[derive(Default)]
struct Segs<'a> {
    base: Option<Words<'a>>,
    alert: Option<Words<'a>>,
    interval: Option<Words<'a>>,
    interval_end: Option<Words<'a>>,
    end: Option<Words<'a>>,
}

fn split_segs<'a>(words: &Words<'a>) -> Segs<'a> {
    enum Marker {
        Base,
        Alert,
        Interval,
        IntervalEnd,
        End,
    }

    let mut base = Words {
        original: words.original,
        words: vec![],
    };
    let mut alert = Words {
        original: words.original,
        words: vec![],
    };
    let mut interval = Words {
        original: words.original,
        words: vec![],
    };
    let mut interval_end = Words {
        original: words.original,
        words: vec![],
    };
    let mut end = Words {
        original: words.original,
        words: vec![],
    };
    let mut marker = Marker::Base;

    for w in words.iter() {
        if w.starts_with("*=") {
            marker = Marker::IntervalEnd;
        } else if w.starts_with("**") || w.starts_with('*') {
            marker = Marker::Interval;
        } else if w.starts_with('=') {
            marker = Marker::End;
        } else if w.starts_with(',') {
            marker = Marker::Alert;
        }

        match marker {
            Marker::Base => base.words.push(*w),
            Marker::Alert => alert.words.push(*w),
            Marker::Interval => interval.words.push(*w),
            Marker::IntervalEnd => interval_end.words.push(*w),
            Marker::End => end.words.push(*w),
        }
    }

    fn non_empty<'a>(words: Words<'a>) -> Option<Words<'a>> {
        if words.is_empty() { None } else { Some(words) }
    }

    Segs {
        base: non_empty(base),
        alert: non_empty(alert),
        interval: non_empty(interval),
        interval_end: non_empty(interval_end),
        end: non_empty(end),
    }
}

fn parse_interval(input: Option<&Words>) -> AResult<Option<(TimeInterval, RepeatType)>> {
    let Some(seg) = input else {
        return Ok(None);
    };

    let first = seg
        .first()
        .ok_or_else(|| anyhow::anyhow!("interval segment should not be empty"))?;
    let (repeat_type, prefix) = if first.starts_with("**") {
        (RepeatType::RepeatTodo, "**")
    } else if first.starts_with('*') {
        (RepeatType::RepeatEvent, "*")
    } else {
        anyhow::bail!("interval should start with * or **: {:?}", seg)
    };

    let interval = TimeInterval::try_from_standard(&seg.remove_first_prefix(prefix))?;
    Ok(Some((interval, repeat_type)))
}

fn parse_alert(input: Option<&Words>) -> AResult<Option<TimeInterval>> {
    if let Some(seg) = input {
        Ok(Some(TimeInterval::try_from_standard(
            &seg.remove_first_prefix(","),
        )?))
    } else {
        Ok(None)
    }
}

fn parse_interval_end(input: Option<&Words>) -> AResult<Option<EndCondition>> {
    if let Some(seg) = input {
        Ok(Some(EndCondition::try_from_standard(
            &seg.remove_first_prefix("*"),
        )?))
    } else {
        Ok(None)
    }
}

fn parse_end(input: Option<&Words>) -> AResult<Option<EndCondition>> {
    if let Some(seg) = input {
        Ok(Some(EndCondition::try_from_standard(seg)?))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test_guess {
    use chrono::FixedOffset;

    use crate::krate::toent::{
        logic::{EventBuilder, timeevent::TimeEvent},
        timeevent::{
            repeater::{
                RepeatType,
                endconditon::{EndCondition, Times},
                interval::TimeInterval,
            },
            timeenum::{TimeEnum, base::BaseDateTime, westen::WesTime},
        },
    };

    fn guess_compare(guess: &str, event: TimeEvent) {
        let guesses = TimeEvent::guess(&guess.into()).unwrap();
        assert_eq!(guesses.get(0).unwrap().timestamp.clone(), event)
    }

    #[test]
    fn test_guess_full() {
        let wes20251226_120000 = WesTime {
            offset: None,
            timestamp: BaseDateTime::new(2025, 12, 26, 12, 00, 00),
        };
        let wes20251226_120000_p800 = WesTime {
            offset: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
            ..wes20251226_120000.clone()
        };

        let alert_15min = TimeInterval {
            base: BaseDateTime::new(None, None, None, None, Some(15), None),
            week: None::<i32>.into(),
        };

        let interval_12h = TimeInterval {
            base: BaseDateTime::new(None, None, None, Some(12), None, None),
            week: None::<i32>.into(),
        };
        let interval_1h_end = EndCondition::Interval(TimeInterval {
            base: BaseDateTime::new(None, None, None, Some(1), None, None),
            week: None::<i32>.into(),
        });

        let end_wes20251227_120000 = EndCondition::Time(TimeEnum::Wes(WesTime {
            offset: None,
            timestamp: BaseDateTime::new(2025, 12, 27, 12, 00, 00),
        }));

        guess_compare(
            "2025-12-26 12:00:00 +8:00 ,15M **12H *=1H =2025-12-27 12:00:00",
            TimeEvent {
                start: Some(TimeEnum::Wes(wes20251226_120000_p800.clone())),
                interval: (interval_12h.clone(), RepeatType::RepeatTodo).into(),
                interval_end: interval_1h_end.clone().into(),
                alert: alert_15min.clone().into(),
                end: end_wes20251227_120000.clone().into(),
            },
        );
    }

    #[test]
    fn test_guess_docs_repeat_todo_with_times_end() {
        guess_compare(
            "2020-12-29 12:13:14 +8:00 **3w =12t",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
                    timestamp: BaseDateTime::new(2020, 12, 29, 12, 13, 14),
                })),
                interval: Some((
                    TimeInterval {
                        base: BaseDateTime::empty(),
                        week: Some(3).into(),
                    },
                    RepeatType::RepeatTodo,
                )),
                interval_end: None,
                alert: None,
                end: Some(EndCondition::Times(Times::new(12))),
            },
        );
    }

    #[test]
    fn test_guess_docs_repeat_event_with_alert_and_deadline() {
        guess_compare(
            "2024-12-12 12:00:00 ,15M *1d =2025-12-12",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: None,
                    timestamp: BaseDateTime::new(2024, 12, 12, 12, 00, 00),
                })),
                interval: Some((
                    TimeInterval {
                        base: BaseDateTime::new(None, None, Some(1), None, None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatEvent,
                )),
                interval_end: None,
                alert: Some(TimeInterval {
                    base: BaseDateTime::new(None, None, None, None, Some(15), None),
                    week: None::<i32>.into(),
                }),
                end: Some(EndCondition::Time(TimeEnum::Wes(WesTime {
                    offset: None,
                    timestamp: BaseDateTime::new(Some(2025), Some(12), Some(12), None, None, None),
                }))),
            },
        );
    }

    #[test]
    fn test_guess_repeat_event_with_interval_end_and_interval_deadline() {
        guess_compare(
            "2024-06-01 09:00:00 *1d *=2H =7d",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: None,
                    timestamp: BaseDateTime::new(2024, 6, 1, 9, 0, 0),
                })),
                interval: Some((
                    TimeInterval {
                        base: BaseDateTime::new(None, None, Some(1), None, None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatEvent,
                )),
                interval_end: Some(EndCondition::Interval(TimeInterval {
                    base: BaseDateTime::new(None, None, None, Some(2), None, None),
                    week: None::<i32>.into(),
                })),
                alert: None,
                end: Some(EndCondition::Interval(TimeInterval {
                    base: BaseDateTime::new(None, None, Some(7), None, None, None),
                    week: None::<i32>.into(),
                })),
            },
        );
    }

    #[test]
    fn test_guess_with_alert_only() {
        guess_compare(
            "2026-01-02 08:30:00 ,10M",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: None,
                    timestamp: BaseDateTime::new(2026, 1, 2, 8, 30, 0),
                })),
                interval: None,
                interval_end: None,
                alert: Some(TimeInterval {
                    base: BaseDateTime::new(None, None, None, None, Some(10), None),
                    week: None::<i32>.into(),
                }),
                end: None,
            },
        );
    }

    #[test]
    fn test_guess_repeat_todo_with_interval_end_times() {
        guess_compare(
            "2024-10-10 07:15:00 **2d *=3t",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: None,
                    timestamp: BaseDateTime::new(2024, 10, 10, 7, 15, 0),
                })),
                interval: Some((
                    TimeInterval {
                        base: BaseDateTime::new(None, None, Some(2), None, None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatTodo,
                )),
                interval_end: Some(EndCondition::Times(Times::new(3))),
                alert: None,
                end: None,
            },
        );
    }

    #[test]
    fn test_guess_with_timezone_and_times_end() {
        guess_compare(
            "2024-03-01 18:45:30 +5:30 *12H =5t",
            TimeEvent {
                start: Some(TimeEnum::Wes(WesTime {
                    offset: FixedOffset::east_opt(5 * 3600 + 30 * 60).map(|e| e.local_minus_utc()),
                    timestamp: BaseDateTime::new(2024, 3, 1, 18, 45, 30),
                })),
                interval: Some((
                    TimeInterval {
                        base: BaseDateTime::new(None, None, None, Some(12), None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatEvent,
                )),
                interval_end: None,
                alert: None,
                end: Some(EndCondition::Times(Times::new(5))),
            },
        );
    }
}
