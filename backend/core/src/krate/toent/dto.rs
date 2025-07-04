use serde::{Deserialize, Serialize};

use super::PossibleToent;

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp {
    pub toents: Vec<PossibleToent>,
}
