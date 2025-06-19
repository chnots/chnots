use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static HASHTAG_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\s#([^\s#\[\]]+)");

pub(crate) fn get_hashtags(input: &str) -> Vec<&str> {
    let mut result = vec![];
    for cap in HASHTAG_REGEX.captures_iter(input) {
        if let Some(hashtag) = cap.get(1) {
            result.push(hashtag.as_str())
        }
    }
    result.into_iter().unique().collect()
}
