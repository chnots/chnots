use std::collections::HashMap;

use chin_sql::time_type::TID;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    krate::graph::{ExcalidrawDataV2, ExcalidrawDataV2Po},
    util::digestutil::blake3_sum,
};

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
        let mut others = HashMap::new();
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
        let mut data = HashMap::new();
        let mut elements_key = vec![];
        for ele in value.0.elements {
            let cell = ele.to_string();
            let sid = blake3_sum(&cell)?;
            data.insert(sid.clone(), cell);
            elements_key.push(sid);
        }
        let mut others_key = HashMap::new();
        for (k, v) in value.0.others {
            let cell = v.to_string();
            let sid = blake3_sum(&cell)?;
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

fn dto_from_string<'de, D>(deserializer: D) -> Result<ExcalidrawDataV2Dto, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Value = Deserialize::deserialize(deserializer)?;
    let s = s.as_str().ok_or(de::Error::custom("empty body"))?;
    let v: Value = serde_json::from_str(s).map_err(|e| de::Error::custom(e.to_string()))?;
    let c = ExcalidrawDataV2Dto::deserialize(&v).map_err(|e| de::Error::custom(e.to_string()))?;
    Ok(c)
}
#[derive(Debug, Clone, Deserialize)]
pub struct ExcalidrawCommitReq {
    pub otid: TID,
    #[serde(deserialize_with = "dto_from_string")]
    pub data: ExcalidrawDataV2Dto,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExcalidrawCommitRsp {}

#[derive(Debug, Clone, Deserialize)]
pub struct ExcalidrawFetchReq {
    pub otid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExcalidrawFetchRsp {
    pub data: Option<ExcalidrawDataV2Dto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MindElixirCommitReq {
    pub otid: TID,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct MindElixirCommitRsp {}

#[derive(Debug, Clone, Deserialize)]
pub struct MindElixirLoadReq {
    pub otid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MindElixirLoadRsp {
    pub data: Value,
}
