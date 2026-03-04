use std::collections::BTreeMap;

use serde::{Deserialize, Serialize, de};
use serde_json::Value;

use crate::{krate::graph::GetKeys, util::digestutil::blake3_sum16};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcalidrawDataV2<T> {
    #[serde(flatten)]
    pub others: BTreeMap<String, T>,
    pub elements: Vec<T>,
}

pub struct ExcalidrawDataV2Po {
    pub meta: ExcalidrawDataV2<String>,
    pub data: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcalidrawLibraryMetaV1 {
    pub sid: String,
}

impl GetKeys for ExcalidrawDataV2<String> {
    fn get_keys(&self) -> Vec<String> {
        let mut keys = vec![];
        keys.extend(self.others.values().map(|e| e.to_string()).to_owned());
        keys.extend(self.elements.clone());
        keys
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ExcalidrawDataV2Dto(pub ExcalidrawDataV2<Value>);

impl<'de> Deserialize<'de> for ExcalidrawDataV2Dto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let body = serde_json::Value::deserialize(deserializer)?;
        let elements = body
            .get("elements")
            .ok_or(de::Error::custom("unable to find elements"))?;
        let elem_arr = elements
            .as_array()
            .ok_or(de::Error::custom("elements is as array"))?;

        let full = body.as_object().ok_or(de::Error::custom("body"))?;
        let mut others = BTreeMap::new();
        for (k, v) in full {
            if k == "elements" {
                continue;
            }
            others.insert(k.to_string(), v.to_owned());
        }

        Ok(Self(ExcalidrawDataV2 {
            others,
            elements: elem_arr.clone(),
        }))
    }
}

impl TryFrom<ExcalidrawDataV2Dto> for ExcalidrawDataV2Po {
    type Error = anyhow::Error;

    fn try_from(value: ExcalidrawDataV2Dto) -> Result<Self, Self::Error> {
        let mut data = BTreeMap::new();
        let mut elements_key = vec![];
        for ele in value.0.elements {
            let cell = ele.to_string();
            let sid = blake3_sum16(cell.as_bytes())?;
            data.insert(sid.clone(), cell);
            elements_key.push(sid);
        }
        let mut others_key = BTreeMap::new();
        for (k, v) in value.0.others {
            let cell = v.to_string();
            let sid = blake3_sum16(cell.as_bytes())?;
            data.insert(sid.clone(), cell);
            others_key.insert(k, sid);
        }

        Ok(ExcalidrawDataV2Po {
            meta: ExcalidrawDataV2 {
                others: others_key,
                elements: elements_key,
            },
            data,
        })
    }
}
