use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait GetKeys {
    fn get_keys(&self) -> Vec<String>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcalidrawDataV2<T> {
    #[serde(flatten)]
    pub others: HashMap<String, T>,
    pub elements: Vec<T>,
}

pub struct ExcalidrawDataV2Po {
    pub meta: ExcalidrawDataV2<String>,
    pub data: HashMap<String, String>,
}

impl GetKeys for ExcalidrawDataV2<String> {
    fn get_keys(&self) -> Vec<String> {
        let mut keys = vec![];
        keys.extend(self.others.values().map(|e| e.to_string()).to_owned());
        keys.extend(self.elements.clone());
        keys
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MindElixirDataV1<T> {
    pub others: HashMap<String, T>,
    pub node_data: Vec<T>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MindElixirNode<T> {
    others: HashMap<String, Value>,
    children: Vec<T>,
}
