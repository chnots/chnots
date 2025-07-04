use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KSpace {
    pub name: String,
    pub color: String,
    pub managers: Vec<String>,
}
