use anyhow::{anyhow, bail};
use chin_sql::time_type::TID;
use chin_tools::AResult;
use serde::{Deserialize, Serialize};
use timeenum::{Timestamp, westen::WesTime};

use crate::krate::toent::{
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
    pub interval_end: Option<TimeInterval>,
    pub alert: Option<TimeInterval>,
    pub end: Option<EndCondition>,
}

#[derive(Clone, Debug, Hash)]
pub struct TimeEventInst {
    pub alert_tid: Option<TID>,
    pub start_tid: Option<TID>,
    pub end_tid: Option<TID>,
    pub is_lunar: bool,
    pub timezone: Option<isize>,
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

    pub(crate) fn generate(
        &self,
        max_count: usize,
        last_finish_uwo: Option<UtcWithOffset>,
        window_start: Option<UtcWithOffset>, // window start
        window_end: Option<UtcWithOffset>,   // window end
    ) -> AResult<Vec<TimeEventInst>> {
        let is_chinese = self.start.is_some_and(|e| matches!(e, TimeEnum::Chn(_)))
            || self
                .end
                .is_some_and(|e| matches!(e, EndCondition::Time(TimeEnum::Chn(_))));

        // get start time
        let start: TimeEnum = match (last_finish_uwo, self.start) {
            (None, None) => bail!("empty start time"),
            (None, Some(start)) => start,
            (Some(start), None) => {
                if is_chinese {
                    TimeEnum::Chn(start.into())
                } else {
                    TimeEnum::Wes(start.into())
                }
            }
            (Some(s1), Some(s2)) => match self.interval.as_ref().map(|s| s.1) {
                Some(RepeatType::RepeatTodo) => {
                    if is_chinese {
                        TimeEnum::Chn(s1.into())
                    } else {
                        TimeEnum::Wes(s1.into())
                    }
                }
                _ => {
                    if s1.utc().cmp(&s2.to_utc_timestamp()?.start().utc()).is_le() {
                        if is_chinese {
                            TimeEnum::Chn(s1.into())
                        } else {
                            TimeEnum::Wes(s1.into())
                        }
                    } else {
                        s2
                    }
                }
            },
        };

        // get end condition
        let mut end_count = max_count;
        let mut end_utc = None;

        if let Some(end_time) = self.end {
            match end_time {
                EndCondition::Times(times) => end_count = end_count.min(times.count() as usize),
                EndCondition::Interval(time_interval) => {
                    end_utc.replace(
                        (self
                            .start
                            .ok_or(anyhow!("end with time_interval but no start_time"))?
                            + time_interval)
                            .ok_or(anyhow!("unable to compute the end time"))?
                            .to_utc_timestamp()?
                            .end(),
                    );
                }
                EndCondition::Time(time_enum) => {
                    end_utc.replace(time_enum.to_utc_timestamp()?.end());
                }
            }
        }
        end_utc = match (end_utc, window_end) {
            (None, None) => None,
            (None, Some(e2)) => e2.into(),
            (Some(e1), None) => e1.into(),
            (Some(s1), Some(e3)) => s1.min(e3).into(),
        };

        let mut result = vec![];
        let mut point = start;

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
            for _c in 0..end_count {
                let point_utc = point.to_utc_timestamp()?;
                let alert_tid = if let Some(interval) = self.alert {
                    Some(
                        (point_utc.start() - interval)
                            .ok_or(anyhow!("unable to  get alert tid"))?
                            .utc(),
                    )
                } else {
                    None
                };
                let end_tid = if let Some(interval) = self.interval_end {
                    Some(
                        (point_utc.start() + interval)
                            .ok_or(anyhow!("unable to  get interval end tid"))?
                            .utc(),
                    )
                } else {
                    None
                };

                let matched = match (window_start, end_utc) {
                    (None, None) => true,
                    (None, Some(e)) => {
                        point_utc.start().utc() <= e.utc()
                            || alert_tid.is_some_and(|a| a <= e.utc())
                    }
                    (Some(s), None) => {
                        s.utc() <= point_utc.end().utc() || alert_tid.is_some_and(|a| s.utc() <= a)
                    }
                    (Some(s), Some(e)) => {
                        (s.utc() <= point_utc.end().utc() && point_utc.end().utc() <= e.utc())
                            || alert_tid.is_some_and(|a| s.utc() <= a && a <= e.utc())
                    }
                };

                if matched {
                    result.push(TimeEventInst {
                        alert_tid: alert_tid,
                        start_tid: Some(point_utc.start().utc()),
                        end_tid: end_tid,
                        is_lunar: is_lunar,
                        timezone: timezone,
                    });
                }
                if end_utc.is_some_and(|end_utc| end_utc.utc() < point_utc.start().utc()) {
                    break;
                }

                point = (point + interval).ok_or(anyhow!("time is to big"))?;
            }
        } else {
            let point_utc = point.to_utc_timestamp()?;
            let alert_tid = if let Some(interval) = self.alert {
                Some(
                    (point_utc.start() - interval)
                        .ok_or(anyhow!("unable to  get alert tid"))?
                        .utc(),
                )
            } else {
                None
            };
            let end_tid = if let Some(interval) = self.end {
                match interval {
                    EndCondition::Times(_) => None,
                    EndCondition::Interval(time_interval) => Some(
                        (point_utc.start() + time_interval)
                            .ok_or(anyhow!("unable to  get interval end tid"))?
                            .utc(),
                    ),
                    EndCondition::Time(time_enum) => {
                        Some(time_enum.to_utc_timestamp()?.end().utc())
                    }
                }
            } else {
                None
            };
            match end_utc {
                Some(end_utc) => {
                    if end_utc.utc() > point_utc.start().utc() {
                        result.push(TimeEventInst {
                            alert_tid: alert_tid,
                            start_tid: Some(point_utc.start().utc()),
                            end_tid: end_tid,
                            is_lunar: is_lunar,
                            timezone: timezone,
                        });
                    }
                }
                None => {
                    result.push(TimeEventInst {
                        alert_tid: alert_tid,
                        start_tid: Some(point_utc.start().utc()),
                        end_tid: end_tid,
                        is_lunar: is_lunar,
                        timezone: timezone,
                    });
                }
            }
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

fn parse_interval_end(input: Option<&Words>) -> AResult<Option<TimeInterval>> {
    if let Some(seg) = input {
        Ok(Some(TimeInterval::try_from_standard(
            &seg.remove_first_prefix("*="),
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
            timeenum::{TimeEnum, base::DymdHMS, westen::WesTime},
        },
    };

    fn guess_compare(guess: &str, event: TimeEvent) {
        let guesses = TimeEvent::guess(&guess.into()).unwrap();
        assert_eq!(guesses.get(0).unwrap().timestamp.clone(), event)
    }

    #[test]
    fn test_guess_full() {
        let wes20251226_120000 = WesTime {
            local_minus_utc: None,
            timestamp: DymdHMS::new(2025, 12, 26, 12, 00, 00),
        };
        let wes20251226_120000_p800 = WesTime {
            local_minus_utc: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
            ..wes20251226_120000.clone()
        };

        let alert_15min = TimeInterval {
            base: DymdHMS::new(None, None, None, None, Some(15), None),
            week: None::<i32>.into(),
        };

        let interval_12h = TimeInterval {
            base: DymdHMS::new(None, None, None, Some(12), None, None),
            week: None::<i32>.into(),
        };
        let interval_1h_end = TimeInterval {
            base: DymdHMS::new(None, None, None, Some(1), None, None),
            week: None::<i32>.into(),
        };

        let end_wes20251227_120000 = EndCondition::Time(TimeEnum::Wes(WesTime {
            local_minus_utc: None,
            timestamp: DymdHMS::new(2025, 12, 27, 12, 00, 00),
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
                    local_minus_utc: FixedOffset::east_opt(8 * 3600).map(|e| e.local_minus_utc()),
                    timestamp: DymdHMS::new(2020, 12, 29, 12, 13, 14),
                })),
                interval: Some((
                    TimeInterval {
                        base: DymdHMS::empty(),
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
                    local_minus_utc: None,
                    timestamp: DymdHMS::new(2024, 12, 12, 12, 00, 00),
                })),
                interval: Some((
                    TimeInterval {
                        base: DymdHMS::new(None, None, Some(1), None, None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatEvent,
                )),
                interval_end: None,
                alert: Some(TimeInterval {
                    base: DymdHMS::new(None, None, None, None, Some(15), None),
                    week: None::<i32>.into(),
                }),
                end: Some(EndCondition::Time(TimeEnum::Wes(WesTime {
                    local_minus_utc: None,
                    timestamp: DymdHMS::new(Some(2025), Some(12), Some(12), None, None, None),
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
                    local_minus_utc: None,
                    timestamp: DymdHMS::new(2024, 6, 1, 9, 0, 0),
                })),
                interval: Some((
                    TimeInterval {
                        base: DymdHMS::new(None, None, Some(1), None, None, None),
                        week: None::<i32>.into(),
                    },
                    RepeatType::RepeatEvent,
                )),
                interval_end: Some(TimeInterval {
                    base: DymdHMS::new(None, None, None, Some(2), None, None),
                    week: None::<i32>.into(),
                }),
                alert: None,
                end: Some(EndCondition::Interval(TimeInterval {
                    base: DymdHMS::new(None, None, Some(7), None, None, None),
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
                    local_minus_utc: None,
                    timestamp: DymdHMS::new(2026, 1, 2, 8, 30, 0),
                })),
                interval: None,
                interval_end: None,
                alert: Some(TimeInterval {
                    base: DymdHMS::new(None, None, None, None, Some(10), None),
                    week: None::<i32>.into(),
                }),
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
                    local_minus_utc: FixedOffset::east_opt(5 * 3600 + 30 * 60)
                        .map(|e| e.local_minus_utc()),
                    timestamp: DymdHMS::new(2024, 3, 1, 18, 45, 30),
                })),
                interval: Some((
                    TimeInterval {
                        base: DymdHMS::new(None, None, None, Some(12), None, None),
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

#[cfg(test)]
mod test_generate {
    use crate::krate::toent::{
        logic::timeevent::TimeEvent,
        timeevent::{
            repeater::{RepeatType, interval::TimeInterval},
            timeenum::{
                TimeEnum, UtcWithOffset,
                base::{DHMS, Dymd, DymdHMS, NoneOrI32},
                westen::WesTime,
            },
        },
    };

    #[test]
    fn test_generate_interval_end_tid_not_shifted_by_timezone() {
        let none = NoneOrI32::from(None::<i32>);
        let event = TimeEvent {
            start: Some(TimeEnum::Wes(WesTime {
                local_minus_utc: None,
                timestamp: DymdHMS {
                    date: Dymd {
                        year: 2026.into(),
                        month: 3.into(),
                        day: 17.into(),
                    },
                    time: DHMS {
                        hour: 21.into(),
                        minute: 0.into(),
                        second: 0.into(),
                    },
                },
            })),
            interval: Some((
                TimeInterval {
                    base: DymdHMS {
                        date: Dymd {
                            year: none,
                            month: none,
                            day: 1.into(),
                        },
                        time: DHMS {
                            hour: none,
                            minute: none,
                            second: none,
                        },
                    },
                    week: none,
                },
                RepeatType::RepeatEvent,
            )),
            interval_end: None,
            alert: Some(TimeInterval {
                base: DymdHMS {
                    date: Dymd {
                        year: none,
                        month: none,
                        day: none,
                    },
                    time: DHMS {
                        hour: none,
                        minute: 15.into(),
                        second: none,
                    },
                },
                week: none,
            }),
            end: Some(
                crate::krate::toent::timeevent::repeater::endconditon::EndCondition::Time(
                    TimeEnum::Wes(WesTime {
                        local_minus_utc: None,
                        timestamp: DymdHMS {
                            date: Dymd {
                                year: 2026.into(),
                                month: 3.into(),
                                day: 20.into(),
                            },
                            time: DHMS {
                                hour: none,
                                minute: none,
                                second: none,
                            },
                        },
                    }),
                ),
            ),
        };

        let result = event
            .generate(
                100,
                None,
                Some(UtcWithOffset::from_utc(
                    1773849600000000i64.try_into().unwrap(),
                )),
                Some(UtcWithOffset::from_utc(
                    1773935999999000i64.try_into().unwrap(),
                )),
            )
            .unwrap();

        println!("{:?}", result);
        assert!(
            result.len() == 1
                && result
                    .iter()
                    .any(|e| e.start_tid.is_some_and(|v| v.as_num() == 1773925200000000))
        )
    }
}
