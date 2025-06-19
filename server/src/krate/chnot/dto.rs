use chin_sql::time_type::TID;
use serde::{Deserialize, Serialize};

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
    pub(crate) kspace: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ChnotTagSearchType {
    Inset(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotQueryReq {
    pub(crate) query: Option<String>,
    pub(crate) meta_tid: Option<TID>,
    pub(crate) record_tid: Option<TID>,

    pub(crate) tags: Option<ChnotTagSearchType>,
    pub(crate) kinds: Vec<ChnotKind>,

    pub(crate) with_omitted: Option<bool>,
    pub(crate) with_archive: Option<bool>,

    // Paging
    pub(crate) start_index: usize,
    pub(crate) page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotQueryRsp<T> {
    pub(crate) data: Vec<T>,
    pub(crate) has_next: bool,
    pub(crate) next_start: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotTagQueryReq {
    pub(crate) query: Option<String>,
    pub(crate) tags: Option<ChnotTagSearchType>,
    pub(crate) remove_params: Option<bool>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotKindRelQueryReq {
    pub(crate) meta_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChnotKindRelQueryRsp {
    pub(crate) kind_rel: ChnotKindRel,
}
