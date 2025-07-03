use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct KSpace {
    pub(crate) name: String,
    pub(crate) color: String,
    pub(crate) managers: Vec<String>,
}
