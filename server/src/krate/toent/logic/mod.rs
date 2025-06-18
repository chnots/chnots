use std::ops::Deref;

use self::eventenum::EventEnum;

pub(crate) mod eventenum;
pub(crate) mod timeevent;
pub(crate) mod todoevent;
use chin_tools::wrapper::score::PossibleScore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Span<'a> {
    text: &'a str,
    start_in: usize,
    end_ex: usize,
}

impl<'a> Deref for Span<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RawInputSegs<'a> {
    pub(crate) original: &'a str,
    pub(crate) spans: Vec<Span<'a>>,
}

impl Default for RawInputSegs<'_> {
    fn default() -> Self {
        Self {
            original: "",
            spans: Default::default(),
        }
    }
}

impl<'a> RawInputSegs<'a> {
    pub(crate) fn sub_start(&self, start: usize) -> RawInputSegs<'a> {
        RawInputSegs {
            original: self.original,
            spans: self.spans.as_slice()[start..].into(),
        }
    }

    pub(crate) fn remove_first_prefix(&self, key: &str) -> RawInputSegs<'a> {
        let mut other = self.clone();
        let first = other.spans.get_mut(0);
        if let Some(f) = first {
            if f.starts_with(key) {
                f.text = &f.text[key.len()..]
            }
        }
        other
    }

    pub(crate) fn sub_range(&self, start: usize, end: usize) -> Self {
        RawInputSegs {
            original: self.original,
            spans: self
                .spans
                .iter()
                .filter(|s| s.start_in >= start && s.end_ex <= end)
                .copied()
                .collect(),
        }
    }

    pub fn empty() -> Self {
        Self {
            original: "",
            spans: vec![],
        }
    }
}

impl<'a> From<&'a str> for RawInputSegs<'a> {
    fn from(input: &'a str) -> Self {
        let mut spans = Vec::new();
        let mut start = 0;
        for (i, c) in input.char_indices() {
            if c == ' ' {
                let word = &input[start..i];
                if !word.is_empty() {
                    spans.push(Span {
                        text: word,
                        start_in: start,
                        end_ex: i,
                    });
                }
                start = i + 1;
            }
        }
        if start < input.len() {
            spans.push(Span {
                text: &input[start..],
                start_in: start,
                end_ex: input.len(),
            });
        }

        RawInputSegs {
            original: input,
            spans,
        }
    }
}

impl<'a> Deref for RawInputSegs<'a> {
    type Target = [Span<'a>];

    fn deref(&self) -> &Self::Target {
        self.spans.as_slice()
    }
}

impl<'a> AsRef<str> for RawInputSegs<'a> {
    fn as_ref(&self) -> &str {
        self.original
    }
}

impl<'a> RawInputSegs<'a> {
    fn full_contains_ig_case(&self, segs: &[&str]) -> bool {
        let lower = self.original.to_ascii_lowercase();
        segs.iter().any(|e| lower.contains(&e.to_lowercase()))
    }

    fn filter<F>(&self, mut filter: F) -> Self
    where
        F: FnMut(&str) -> bool,
    {
        RawInputSegs {
            original: self.original,
            spans: self
                .spans
                .iter()
                .filter(|e| filter(e.text))
                .copied()
                .collect(),
        }
    }
}

pub(crate) trait EventBuilder
where
    Self: Sized,
{
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>>;

    fn is_valid(&self) -> bool;

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self>;
    fn standard_str(&self) -> String;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct PossibleToent {
    input: String,
    event: EventEnum,
}

impl PossibleToent {
    pub(crate) fn from_standard(input: &str) -> anyhow::Result<PossibleToent> {
        Ok(PossibleToent {
            input: input.to_owned(),
            event: EventEnum::from_standard(&RawInputSegs::from(input))?,
        })
    }

    pub(crate) fn guess(input: &str) -> Vec<PossibleToent> {
        if let Some(mut guesses) = EventEnum::guess(&input.into()) {
            guesses.sort_by(|e1, e2| e2.1.partial_cmp(&e1.1).unwrap_or(std::cmp::Ordering::Equal));

            guesses
                .into_iter()
                .map(|e| PossibleToent {
                    input: input.to_owned(),
                    event: e.0,
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use super::PossibleToent;

    fn print_and_compare<T: Debug + PartialEq>(t1: T, t2: T) {
        println!("===================");
        println!("t1: {t1:?}");
        println!("t2: {t2:?}");
        assert!(t1 == t2)
    }
    fn t_same(guess: &str, standard: &str) {
        print_and_compare(
            PossibleToent::guess(guess).first().map(|e| &e.event),
            PossibleToent::from_standard(standard)
                .ok()
                .as_ref()
                .map(|e| &e.event),
        );
    }

    #[test]
    fn test() {
        t_same("done", "DONE");
        t_same("ns 2025-12-26", "农 2025-12-26");
        t_same("ns 2025-12-26 =10d", "农 2025-12-26 =10d");
        t_same(
            "2025-12-26 12:00:00 +8:00 =2025-12-27 12:00:00",
            "2025-12-26 12:00:00 +8:00 =2025-12-27 12:00:00",
        );
        println!("{:?}", PossibleToent::guess("2502-12"))
    }
}
