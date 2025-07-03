use serde::{Deserialize, Serialize};

use super::PossibleToent;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ToentGuessReq {
    pub(crate) input: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ToentGuessRsp {
    pub(crate) toents: Vec<PossibleToent>,
}
