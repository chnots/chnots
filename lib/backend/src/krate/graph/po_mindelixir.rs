use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use chin_tools::{AResult, SharedStr};
use serde::{Deserialize, Serialize, de};
use serde_json::Value;

use crate::util::digestutil::blake3_sum;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MindElixirDataV1<NT, T> {
    #[serde(flatten)]
    pub others: HashMap<String, T>,
    pub node_data: NT,
    pub arrows: Option<Vec<T>>,
    pub summaries: Option<Vec<T>>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MindElixirDataV2Dto(pub MindElixirDataV1<MindElixirNode, Value>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MindElixirDataV1PoMeta {
    pub meta: MindElixirDataV1<String, String>,
    pub keys: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MindElixirDataV1Po {
    pub meta: MindElixirDataV1PoMeta,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MindElixirNode {
    #[serde(flatten)]
    others: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Box<Self>>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MindElixirNodeStr {
    #[serde(flatten)]
    others: HashMap<String, Value>,
    children: Option<Vec<SharedStr>>,
}

impl<'de> Deserialize<'de> for MindElixirNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let body = serde_json::Value::deserialize(deserializer)?;
        let children = body.get("children");

        let full = body.as_object().ok_or(de::Error::custom("body"))?;
        let mut others = HashMap::new();
        for (k, v) in full {
            if k == "children" {
                continue;
            }
            others.insert(k.to_string(), v.to_owned());
        }

        let children = if let Some(children) = children {
            let mut result = vec![];
            for child in children
                .as_array()
                .ok_or(de::Error::custom("children is not array"))?
            {
                let c: MindElixirNode = serde_json::from_value(child.clone())
                    .map_err(|err| de::Error::custom(err.to_string()))?;
                result.push(c.into());
            }
            Some(result)
        } else {
            None
        };

        Ok(Self { others, children })
    }
}

impl MindElixirNode {
    pub fn flattern(&self) -> AResult<(SharedStr, HashMap<SharedStr, String>)> {
        let mut result_map = HashMap::new();
        let sid = self.flatten_inner(&mut result_map)?;
        Ok((sid, result_map))
    }

    fn flatten_inner(&self, result_map: &mut HashMap<SharedStr, String>) -> AResult<SharedStr> {
        let children = &self.children;
        let cks = match children {
            Some(children) => {
                let mut keys = vec![];
                for node in children {
                    let key = node.flatten_inner(result_map)?;
                    keys.push(key);
                }
                Some(keys)
            }
            None => None,
        };
        let c = MindElixirNodeStr {
            others: self.others.clone(),
            children: cks,
        };

        let leaf = serde_json::to_string(&c)?;
        let sid: SharedStr = blake3_sum(&leaf)?.into();

        result_map.insert(sid.clone(), leaf);
        Ok(sid)
    }
}

impl<'de> Deserialize<'de> for MindElixirDataV2Dto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let body = serde_json::Value::deserialize(deserializer)?;
        let node_data = body
            .get("nodeData")
            .ok_or(de::Error::custom("unable to find nodeData"))?;

        let node_data: MindElixirNode =
            serde_json::from_value(node_data.clone()).map_err(|err| de::Error::custom(err))?;

        let arrows = body.get("arrows");
        let arrows = if let Some(arrows) = arrows {
            arrows
                .as_array()
                .map(|a| a.iter().map(|e| e.clone()).collect())
        } else {
            None
        };
        let summaries = body.get("summaries");
        let summaries = if let Some(summaries) = summaries {
            summaries
                .as_array()
                .map(|a| a.iter().map(|e| e.clone()).collect())
        } else {
            None
        };

        let full = body.as_object().ok_or(de::Error::custom("body"))?;
        let mut others = HashMap::new();
        for (k, v) in full {
            if k == "nodeData" || k == "arrows" || k == "summaries" {
                continue;
            }
            others.insert(k.to_string(), v.to_owned());
        }

        Ok(MindElixirDataV2Dto(MindElixirDataV1 {
            others: others,
            node_data: node_data,
            arrows: arrows,
            summaries: summaries,
        }))
    }
}

impl TryFrom<MindElixirDataV2Dto> for MindElixirDataV1Po {
    type Error = anyhow::Error;

    fn try_from(value: MindElixirDataV2Dto) -> Result<Self, Self::Error> {
        let mut data = HashMap::new();

        let mut others_key = HashMap::new();
        for (k, v) in value.0.others {
            let cell = v.to_string();
            let sid = blake3_sum(&cell)?;
            data.insert(sid.clone(), cell);
            others_key.insert(k, sid);
        }

        let arrows = if let Some(vs) = value.0.arrows {
            let mut values_key = vec![];
            for ele in vs {
                let v = serde_json::to_string(&ele)?;
                let sid = blake3_sum(&v)?;
                data.insert(sid.clone(), v);
                values_key.push(sid);
            }
            Some(values_key)
        } else {
            None
        };

        let summaries = if let Some(vs) = value.0.summaries {
            let mut values_key = vec![];
            for ele in vs {
                let v = serde_json::to_string(&ele)?;
                let sid = blake3_sum(&v)?;
                data.insert(sid.clone(), v);
                values_key.push(sid);
            }
            Some(values_key)
        } else {
            None
        };

        let (node_data_key, value) = value.0.node_data.flattern()?;
        data.extend(value.into_iter().map(|(e, k)| (e.to_string(), k)));

        Ok(MindElixirDataV1Po {
            meta: MindElixirDataV1PoMeta {
                meta: MindElixirDataV1 {
                    others: others_key,
                    node_data: node_data_key.to_string(),
                    arrows,
                    summaries,
                },
                keys: data.keys().map(|e| e.to_string()).collect(),
            },
            data: data,
        })
    }
}

fn depth(root: SharedStr, data: &HashMap<String, String>) -> AResult<MindElixirNode> {
    let v = data
        .get(root.as_str())
        .ok_or(anyhow!("unable to get {}", root))?;
    let d: MindElixirNodeStr = serde_json::from_str(v)?;
    let children = match d.children {
        Some(v) => {
            let mut children = vec![];
            for ele in v {
                let child = depth(ele, data)?;
                children.push(child.into());
            }
            Some(children)
        }
        None => None,
    };
    Ok(MindElixirNode {
        others: d.others.clone(),
        children: children,
    })
}

impl TryFrom<MindElixirDataV1Po> for MindElixirDataV2Dto {
    type Error = anyhow::Error;

    fn try_from(value: MindElixirDataV1Po) -> Result<Self, Self::Error> {
        let MindElixirDataV1Po { meta, data } = value;

        let meta = meta.meta;
        let mut others = HashMap::new();
        for (k, v) in meta.others {
            let v = data
                .get(v.as_str())
                .ok_or(anyhow::anyhow!("unable get {}", v.as_str()))?;

            others.insert(k.to_string(), serde_json::from_str(v)?);
        }

        let arrows = if let Some(vs) = &meta.arrows {
            let mut res = vec![];
            for s in vs.iter() {
                let v = data
                    .get(s.as_str())
                    .ok_or(anyhow::anyhow!("unable get {}", s.as_str()))?;
                res.push(serde_json::from_str(v)?);
            }
            Some(res)
        } else {
            None
        };

        let summaries = if let Some(vs) = &meta.summaries {
            let mut res = vec![];
            for s in vs.iter() {
                let v = data
                    .get(s.as_str())
                    .ok_or(anyhow::anyhow!("unable get {}", s.as_str()))?;
                res.push(serde_json::from_str(v)?);
            }
            Some(res)
        } else {
            None
        };

        Ok(Self(MindElixirDataV1 {
            others: others,
            node_data: depth(meta.node_data.into(), &data)?,
            arrows: arrows,
            summaries: summaries,
        }))
    }
}
