use std::collections::HashMap;

use chin_sql::time_type::TID;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{krate::graph::*, util::digestutil::blake3_sum};

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
    pub data: MindElixirDataV2Dto,
}

#[derive(Debug, Clone, Serialize)]
pub struct MindElixirCommitRsp {}

#[derive(Debug, Clone, Deserialize)]
pub struct MindElixirLoadReq {
    pub otid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MindElixirLoadRsp {
    pub data: Option<MindElixirDataV2Dto>,
}
