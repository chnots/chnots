use crate::toent::PossibleToent;
use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Serialize};

use crate::model::db::chnot::{ChnotKind, ChnotMetadata, ChnotRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub record: ChnotRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub meta_id: String,
    pub workspace: Option<String>,
    pub update_time: bool,
    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteReq {
    pub id: Option<String>,
    pub meta_id: Option<String>,
    pub content: String,
    pub kind: ChnotKind,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteRsp {
    pub chnot: Chnot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionReq {
    pub chnot_id: String,
    /// logic or physical deletion
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionRsp {}

#[derive(Debug, Clone, Serialize)]
pub enum ChnotTagTreeType {
    Children(String),
    Descendants(String),
}

impl ChnotTagTreeType {
    pub fn is_empty(&self) -> bool {
        match self {
            ChnotTagTreeType::Children(prefix) => prefix.is_empty(),
            ChnotTagTreeType::Descendants(prefix) => prefix.is_empty(),
        }
    }

    pub fn path(&self) -> &str {
        match self {
            ChnotTagTreeType::Children(prefix) => &prefix,
            ChnotTagTreeType::Descendants(prefix) => &prefix,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ChnotViewType {
    Timeline,
    TagTree(ChnotTagTreeType),
}

impl<'a> Deserialize<'a> for ChnotViewType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        #[derive(Deserialize)]
        struct LVT {
            kind: String,
            tagkind: Option<String>,
            tagpath: Option<String>,
        }
        let deser = LVT::deserialize(deserializer)?;

        match deser.kind.as_str() {
            "tagtree" => match deser.tagkind.as_ref().map(|e| e.as_str()) {
                Some("children") => Ok(Self::TagTree(ChnotTagTreeType::Children(
                    deser.tagpath.unwrap(),
                ))),
                Some("descendants") => Ok(Self::TagTree(ChnotTagTreeType::Descendants(
                    deser.tagpath.unwrap(),
                ))),
                _ => Err(de::Error::custom("unknown tag type")),
            },
            "timeline" => Ok(Self::Timeline),
            _ => Err(de::Error::custom("unknown type")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryReq {
    pub query: Option<String>,
    pub meta_id: Option<String>,
    pub record_id: Option<String>,
    pub tag_path: Option<String>,
    pub view_type: ChnotViewType,

    pub with_deleted: Option<bool>,
    pub with_omitted: Option<bool>,
    pub with_archived: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp<T> {
    pub data: T,
    pub has_next: bool,
    pub next_start: usize
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp {
    pub toents: Vec<PossibleToent>,
}

impl<'a> Deserialize<'a> for ChnotTagTreeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        let des = ChnotViewType::deserialize(deserializer)?;
        match des {
            ChnotViewType::Timeline => Err(de::Error::custom("unable map  to TagTree")),
            ChnotViewType::TagTree(chnot_tag_tree_type) => Ok(chnot_tag_tree_type),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagQueryReq {
    pub query: Option<String>,
    pub tag_tree: ChnotTagTreeType,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotTagQueryRsp<T>
where
    T: Serialize + Clone,
{
    pub data: Vec<T>,

    pub start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagUpdateReq {
    pub content: String,
    pub meta_id: String,
    pub workspace: String,
}