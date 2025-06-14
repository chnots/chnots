use chin_sql::time_type::TID;
use serde::{de, Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Chnot {
    pub(crate) record: ChnotRecord,
    pub(crate) meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotUpdateReq {
    pub(crate) meta_tid: TID,
    pub(crate) kspace: Option<String>,
    pub(crate) pinned: Option<bool>,
    pub(crate) archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotOverwriteReq {
    pub(crate) meta_tid: Option<TID>,
    pub(crate) content: String,
    pub(crate) kind: ChnotKind,
    pub(crate) kind_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotOverwriteRsp {
    pub(crate) meta_tid: TID,
    pub(crate) rec_tid: TID,
    pub(crate) ksapce: String,
    pub(crate) archor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotDeletionReq {
    pub(crate) meta_tid: TID,
    /// logic or physical deletion
    pub(crate) logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotDeletionRsp {}

#[derive(Debug, Clone, Serialize)]
pub(crate) enum ChnotTagTreeType {
    Children(String),
    Descendants(String),
}

impl ChnotTagTreeType {
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            ChnotTagTreeType::Children(prefix) => prefix.is_empty(),
            ChnotTagTreeType::Descendants(prefix) => prefix.is_empty(),
        }
    }

    pub(crate) fn path(&self) -> &str {
        match self {
            ChnotTagTreeType::Children(prefix) => prefix,
            ChnotTagTreeType::Descendants(prefix) => prefix,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) enum ChnotViewType {
    Timeline,
    TagTree(ChnotTagTreeType),
}

impl<'a> Deserialize<'a> for ChnotViewType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        #[derive(Deserialize)]
        #[allow(clippy::upper_case_acronyms)]
        struct LVT {
            kind: String,
            tagkind: Option<String>,
            tagpath: Option<String>,
        }
        let deser = LVT::deserialize(deserializer)?;

        match deser.kind.as_str() {
            "tagtree" => match deser.tagkind.as_deref() {
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
pub(crate) struct ChnotQueryReq {
    pub(crate) query: Option<String>,
    pub(crate) meta_tid: Option<TID>,
    pub(crate) record_tid: Option<String>,

    pub(crate) view_type: ChnotViewType,
    pub(crate) kinds: Vec<ChnotKind>,

    pub(crate) with_omitted: Option<bool>,

    // Paging
    pub(crate) start_index: usize,
    pub(crate) page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotQueryRsp<T> {
    pub(crate) data: T,
    pub(crate) has_next: bool,
    pub(crate) next_start: usize,
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
pub(crate) struct ChnotTagQueryReq {
    pub(crate) query: Option<String>,
    pub(crate) tag_tree: ChnotTagTreeType,

    // Paging
    pub(crate) start_index: usize,
    pub(crate) page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ChnotTagQueryRsp<T>
where
    T: Serialize + Clone,
{
    pub(crate) data: Vec<T>,

    pub(crate) start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotTagUpdateReq {
    pub(crate) content: String,
    pub(crate) meta_tid: TID,
    pub(crate) kspace: String,
}
