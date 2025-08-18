use chin_tools::score::PossibleScore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug)]
pub struct GuessElem<T> {
    pub toent: T,
    pub score: PossibleScore,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp<T> {
    pub toents: Vec<T>,
}

impl<T> From<(T, PossibleScore)> for GuessElem<T> {
    fn from(value: (T, PossibleScore)) -> Self {
        GuessElem {
            toent: value.0,
            score: value.1,
        }
    }
}

pub fn toent2<E, V>(value: GuessElem<E>) -> GuessElem<V>
where
    E: Into<V>,
{
    GuessElem {
        toent: value.toent.into(),
        score: value.score,
    }
}
