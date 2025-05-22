use std::{
    ops::{Deref, DerefMut},
    vec,
};

use super::PossibleScore;
use crate::krate::toent::{
    timeevent::timeenum::base::{BaseTime, NoneOrI32},
    EventBuilder, RawInputSegs,
};
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TimeInterval {
    base: BaseTime,
    week: NoneOrI32,
}

impl Deref for TimeInterval {
    type Target = BaseTime;

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
    fn guess(gt: &RawInputSegs) -> Option<Vec<(Self, PossibleScore)>> {
        match Self::from_standard(gt) {
            Ok(v) => Some(vec![(v, PossibleScore::Yes(10))]),
            Err(_) => None,
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(gt: &RawInputSegs) -> anyhow::Result<Self> {
        let mut num = String::new();
        let mut interval = TimeInterval::default();
        for c in gt.spans[0].chars() {
            match c {
                '0'..='9' => num.push(c),
                'y' => {
                    interval.year = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'm' => {
                    interval.month = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'd' => {
                    interval.day = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'H' => {
                    interval.hour = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'M' => {
                    interval.minute = num.parse::<i32>()?.into();
                    num = String::new();
                }
                'S' => {
                    interval.second = num.parse::<i32>()?.into();
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

    fn standard_str(&self) -> String {
        let mut result = String::new();
        let mut push_func = |v: &NoneOrI32, u: char| {
            if let Some(i) = v.as_ref() {
                result.push_str(&i.to_string());
                result.push(u);
            }
        };

        push_func(&self.year, 'y');
        push_func(&self.month, 'm');
        push_func(&self.week, 'w');
        push_func(&self.day, 'd');
        push_func(&self.hour, 'H');
        push_func(&self.minute, 'M');
        push_func(&self.second, 'S');

        result
    }
}

#[cfg(test)]
mod test {

    use crate::krate::toent::{EventBuilder, RawInputSegs};

    use super::TimeInterval;

    #[test]
    fn test() {
        let ti = TimeInterval::from_standard(&RawInputSegs::from("1d2m444w")).unwrap();
        println!("{}", ti.standard_str());
    }
}
