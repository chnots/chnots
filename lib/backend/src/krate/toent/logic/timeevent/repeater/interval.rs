use std::{
    ops::{Add, Deref, DerefMut},
    vec,
};

use chin_tools::AResult;
use serde::{Deserialize, Serialize};

use super::PossibleScore;
use crate::krate::toent::{
    EventBuilder, Words,
    dto::GuessElem,
    timeevent::timeenum::base::{BaseDateTime, NoneOrI32},
};
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub(crate) struct TimeInterval {
    pub(crate) base: BaseDateTime,
    pub(crate) week: NoneOrI32,
}

impl Deref for TimeInterval {
    type Target = BaseDateTime;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for TimeInterval {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EventBuilder for TimeInterval {
    fn guess(gt: &Words) -> Option<Vec<GuessElem<Self>>> {
        match Self::try_from_standard(gt) {
            Ok(v) => Some(vec![(v, PossibleScore::Yes(10)).into()]),
            Err(_) => None,
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn try_from_standard(gt: &Words) -> AResult<Self> {
        let mut num = String::new();
        let mut interval = TimeInterval::default();
        for c in gt.words[0].chars() {
            match c {
                '0'..='9' => num.push(c),
                'y' => {
                    interval.date.year = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'm' => {
                    interval.date.month = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'd' => {
                    interval.date.day = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'H' => {
                    interval.time.hour = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'M' => {
                    interval.time.minute = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'S' => {
                    interval.time.second = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'w' => {
                    interval.week = num.parse::<i32>()?.into();
                    num = String::new();
                }
                '-' => {
                    if num.is_empty() {
                        num.push('-');
                    } else {
                        anyhow::bail!("unable to parse TimeInterval: {}", c);
                    }
                }
                _ => anyhow::bail!("unable to parse TimeInterval: {}", c),
            }
        }
        Ok(interval)
    }

    fn standard_string(&self) -> String {
        let mut result = String::new();
        let mut push_func = |v: &NoneOrI32, u: char| {
            if let Some(i) = v.as_ref() {
                result.push_str(&i.to_string());
                result.push(u);
            }
        };

        push_func(&self.date.year, 'y');
        push_func(&self.date.month, 'm');
        push_func(&self.week, 'w');
        push_func(&self.date.day, 'd');
        push_func(&self.time.hour, 'H');
        push_func(&self.time.minute, 'M');
        push_func(&self.time.second, 'S');

        result
    }
}

const MICROS_PER_SECOND: i64 = 1_000_000;
const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

impl TimeInterval {
    pub fn to_micros(week: i64, day: i64, hour: i64, minute: i64, second: i64) -> i64 {
        let total_seconds = i64::from(day) * SECONDS_PER_DAY
            + i64::from(week) * 7 * SECONDS_PER_DAY
            + i64::from(hour) * 3600
            + i64::from(minute) * 60
            + i64::from(second);
        total_seconds * MICROS_PER_SECOND
    }
}

#[cfg(test)]
mod test {

    use crate::krate::toent::{EventBuilder, Words};

    use super::TimeInterval;

    #[test]
    fn test() {
        let ti = TimeInterval::try_from_standard(&Words::from("1d2m444w")).unwrap();
        println!("{}", ti.standard_string());
    }
}
