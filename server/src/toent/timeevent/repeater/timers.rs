use chin_tools::score::PossibleScore;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::toent::{EventBuilder, RawInputSegs};

#[derive(Clone, Debug, PartialEq)]

pub(crate) struct Times {
    count: u32,
}

static TIMES_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"^(\d+)t$");

impl EventBuilder for Times {
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        match Self::from_standard(&gt) {
            Ok(v) => Some(vec![(v, PossibleScore::Likely(100))]),
            Err(_) => None,
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let segs = &gt.spans;
        if segs.len() != 1 {
            anyhow::bail!("Times segs' count Should be 1: {:?}", segs);
        }

        match TIMES_REGEX.captures(segs[0].text).map(|e| e.get(1)) {
            Some(Some(v)) => Ok(Self {
                count: u32::from_str_radix(v.as_str(), 10)?,
            }),
            _ => {
                anyhow::bail!("unable to parse it: {:?}", segs)
            }
        }
    }

    fn standard_str(&self) -> String {
        format!("{}t", self.count)
    }
}
