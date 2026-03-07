use chin_tools::{AResult, score::PossibleScore};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::krate::toent::{EventBuilder, Words, dto::GuessElem};

#[derive(Clone, Debug, PartialEq)]

pub(crate) struct Times {
    count: u32,
}

impl Times {
    pub(crate) fn new(count: u32) -> Self {
        Self { count }
    }
}

static TIMES_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"^(\d+)t$");

impl EventBuilder for Times {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(v) => Some(vec![GuessElem {
                timestamp: v,
                score: PossibleScore::Likely(100),
            }]),
            Err(_) => None,
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let segs = &gt.words;
        if segs.len() != 1 {
            anyhow::bail!("Times segs' count Should be 1: {:?}", segs);
        }

        match TIMES_REGEX.captures(segs[0].text).map(|e| e.get(1)) {
            Some(Some(v)) => Ok(Self {
                count: v.as_str().parse::<u32>()?,
            }),
            _ => {
                anyhow::bail!("unable to parse it: {:?}", segs)
            }
        }
    }

    fn standard_string(&self) -> String {
        format!("{}t", self.count)
    }
}
