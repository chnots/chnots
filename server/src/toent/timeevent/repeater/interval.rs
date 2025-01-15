use std::{
    ops::{Deref, DerefMut},
    vec,
};

use crate::{model::score::PossibleScore, toent::{timeevent::timeenum::base::{BaseTime, Unit}, EventBuilder, GuessType}};



#[derive(Clone, Debug, Default)]
pub struct TimeInterval {
    base: BaseTime,
    week: Unit,
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
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)> {
        match Self::from_standard(&input.segs) {
            Ok(v) => {
                vec![(v, PossibleScore::Yes(10))]
            }
            Err(_) => {
                vec![]
            }
        }
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        let mut num = String::new();
        let mut interval = TimeInterval::default();
        for c in segs[0].chars() {
            match c {
                '0'..='9' => num.push(c),
                'y' => {
                    interval.year = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'm' => {
                    interval.month = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'd' => {
                    interval.day = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'H' => {
                    interval.hour = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'M' => {
                    interval.minute = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'S' => {
                    interval.second = i32::from_str_radix(&num, 10)?.into();
                    num = String::new();
                }
                'w' => interval.week = i32::from_str_radix(&num, 10)?.into(),

                '-' => {
                    if num.len() == 0 {
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
        let mut push_func = |v: &Unit, u: char| match v.as_ref() {
            Some(i) => {
                result.push_str(&i.to_string());
                result.push(u);
            }
            None => {}
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


    use crate::toent::EventBuilder;

    use super::TimeInterval;

    #[test]
    fn test() {
        let ti = TimeInterval::from_standard(&["1d2m444w"]).unwrap();
        println!("{}", ti.standard_str());
    }
}
