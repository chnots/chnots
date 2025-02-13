use std::ops::Deref;

use crate::model::score::PossibleScore;

use self::{eventenum::EventEnum, timeevent::TimeEvent};

pub mod eventenum;
pub mod timeevent;
pub mod todoevent;

#[inline]
pub fn retain_parts<F>(input: &str, retain_func: F) -> Vec<&str>
where
    F: FnMut(&&str) -> bool,
{
    input
        .split(" ")
        .filter(|e| e.len() > 0)
        .filter(retain_func)
        .collect()
}

#[inline]
pub fn retain_not_empty_parts(input: &str) -> Vec<&str> {
    retain_parts(input, |e| !e.is_empty())
}

pub struct GuessType<'a> {
    original: &'a str,
    segs: Vec<&'a str>,
}

impl<'a> From<&'a str> for GuessType<'a> {
    fn from(value: &'a str) -> Self {
        GuessType {
            original: &value,
            segs: retain_not_empty_parts(&value),
        }
    }
}

impl<'a> Deref for GuessType<'a> {
    type Target = [&'a str];

    fn deref(&self) -> &Self::Target {
        self.segs.as_slice()
    }
}

impl<'a> AsRef<str> for GuessType<'a> {
    fn as_ref(&self) -> &str {
        &self.original
    }
}

impl<'a> GuessType<'a> {
    fn full_contains_ig_case(&self, segs: &[&str]) -> bool {
        let lower = self.original.to_ascii_lowercase();
        segs.iter().any(|e| lower.contains(&e.to_lowercase()))
    }

    fn filter<F>(&self, mut filter: F) -> Self
    where
        F: FnMut(&str) -> bool,
    {
        GuessType {
            original: self.original,
            segs: self.segs.iter().filter(|e| filter(e)).map(|e| *e).collect(),
        }
    }

    fn groups(&self) -> (GuessType, Vec<GuessType>) {
        let (base, repeaters) = TimeEvent::sep_base_and_others(&self.segs);

        (
            GuessType {
                original: &self.original,
                segs: base,
            },
            repeaters
                .into_iter()
                .map(|e| GuessType {
                    original: &self.original,
                    segs: e,
                })
                .collect(),
        )
    }
}

pub trait EventBuilder
where
    Self: Sized,
{
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)>;

    fn is_valid(&self) -> bool;

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self>;
    fn standard_str(&self) -> String;
}

use chin_tools::utils::idutils;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PossibleToent {
    id: String,
    input: String,
    event: EventEnum,
}

impl PossibleToent {
    pub fn from_standard(input: &str) -> anyhow::Result<PossibleToent> {
        let parts = retain_parts(input, |e| !e.is_empty());

        Ok(PossibleToent {
            id: idutils::generate_uuid(),
            input: input.to_owned(),
            event: EventEnum::from_standard(&parts)?,
        })
    }

    pub fn guess(input: &str) -> Vec<PossibleToent> {
        let mut guess_res = EventEnum::guess(&input.into());
        guess_res.sort_by(|e1, e2| e1.1.cmp(&e2.1));

        let res = guess_res
            .into_iter()
            .map(|e| PossibleToent {
                id: idutils::generate_uuid(),
                input: input.to_owned(),
                event: e.0,
            })
            .collect();

        res
    }
}

#[cfg(test)]
mod test {

    use crate::toent::EventBuilder;

    use super::PossibleToent;

    #[test]
    fn test() {
        let r = PossibleToent::from_standard("TODO");
        println!("{:?}", r);

        let r = PossibleToent::from_standard("2024-02-12 12:00:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = PossibleToent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = PossibleToent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = PossibleToent::from_standard("2024-02-12 12:00:00 -8:00 .*5d ,10H =10t **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r =
            PossibleToent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12 **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = PossibleToent::from_standard(
            "2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12-12 12:00 **10d =10d",
        );
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =10m **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12 12:00:00 +8:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12 12:00:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12 12:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12 12");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = PossibleToent::from_standard("2024-02-12");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = PossibleToent::guess("todo");
        println!("{:?}", r);
        let r = PossibleToent::guess("now ..5d ,10H =10m **10d =10d");
        println!("{:?}", r);
    }
}
