use std::ops::Deref;

pub(crate) mod timeevent;
pub(crate) mod todoevent;
use chin_tools::{AResult, wrapper::score::PossibleScore};
use itertools::Itertools;

use crate::krate::toent::dto::GuessElem;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Word<'a> {
    text: &'a str,
    start_in: usize,
    end_ex: usize,
}

impl<'a> Deref for Word<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text
    }
}

/// Words, split user input by spaces.
#[derive(Debug, Clone, Default)]
pub(crate) struct Words<'a> {
    pub(crate) original: &'a str,
    pub(crate) words: Vec<Word<'a>>,
}

impl<'a> Words<'a> {
    pub(crate) fn sub_start(&self, start: usize) -> Words<'a> {
        Words {
            original: self.original,
            words: self.words.as_slice()[start..].into(),
        }
    }

    pub(crate) fn remove_first_prefix(&self, key: &str) -> Words<'a> {
        let mut other = self.clone();
        let first = other.words.get_mut(0);
        if let Some(f) = first
            && f.starts_with(key)
        {
            f.text = &f.text[key.len()..]
        }

        other
    }

    pub(crate) fn sub_range(&self, start: usize, end: usize) -> Self {
        Words {
            original: self.original,
            words: self
                .words
                .iter()
                .filter(|s| s.start_in >= start && s.end_ex <= end)
                .copied()
                .collect(),
        }
    }

    pub fn empty() -> Self {
        Self {
            original: "",
            words: vec![],
        }
    }

    pub fn filterd(&self) -> String {
        self.words.iter().map(|e| e.text).join(" ")
    }

    pub fn sub1(&self, nth: usize) -> Option<Self> {
        self.words.get(nth).map(|n| Self {
            original: self.original,
            words: vec![n.clone()],
        })
    }
}

impl<'a> From<&'a str> for Words<'a> {
    fn from(original: &'a str) -> Self {
        let mut words = Vec::new();
        let mut start = 0;
        for (i, c) in original.char_indices() {
            if c == ' ' {
                let word = &original[start..i];
                if !word.is_empty() {
                    words.push(Word {
                        text: word,
                        start_in: start,
                        end_ex: i,
                    });
                }
                start = i + 1;
            }
        }
        if start < original.len() {
            words.push(Word {
                text: &original[start..],
                start_in: start,
                end_ex: original.len(),
            });
        }

        Self { original, words }
    }
}

impl<'a> Deref for Words<'a> {
    type Target = [Word<'a>];

    fn deref(&self) -> &Self::Target {
        self.words.as_slice()
    }
}

impl<'a> AsRef<str> for Words<'a> {
    fn as_ref(&self) -> &str {
        self.original
    }
}

impl<'a> Words<'a> {
    fn full_contains_ig_case(&self, segs: &[&str]) -> bool {
        let lower = self.original.to_ascii_lowercase();
        segs.iter().any(|e| lower.contains(&e.to_lowercase()))
    }

    fn filter<F>(&self, mut filter: F) -> Self
    where
        F: FnMut(&str) -> bool,
    {
        Words {
            original: self.original,
            words: self
                .words
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
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>>;

    fn is_valid(&self) -> bool;

    fn try_from_standrd_str(s: &str) -> AResult<Self> {
        Self::try_from_standard(&s.into())
    }
    fn try_from_standard(gt: &Words) -> AResult<Self>;
    fn standard_string(&self) -> String;
}
