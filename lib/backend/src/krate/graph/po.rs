use chin_sql::SqlValue;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{GenerateTableSchema, str_type::Text};

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::krate::graph::{ExcalidrawDataV2, ExcalidrawLibraryMetaV1, MindElixirDataV1PoMeta};
use crate::mapper::db::{KDbRow, KDbRowBehavier};
use crate::{enum_common_funcs, impl_otid_support, impl_sid_support};

pub trait GetKeys {
    fn get_keys(&self) -> Vec<String>;
}

#[derive(Debug, Clone, Sequence)]
pub enum GraphKind {
    ExcalidrawV2,
    MindElixirV1,
    ExcalidrawLibraryV1,
}

#[derive(Debug, Clone)]
pub enum GraphMetaEnum {
    ExcalidrawV2(ExcalidrawDataV2<String>),
    MindElixirV1(MindElixirDataV1PoMeta),
    ExcalidrawLibraryV1(ExcalidrawLibraryMetaV1),
}

impl GraphKind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            GraphKind::ExcalidrawV2 => "exdrv2",
            GraphKind::MindElixirV1 => "mielixirv1",
            GraphKind::ExcalidrawLibraryV1 => "exdrlibv1",
        }
    }
}
enum_common_funcs!(GraphKind);

impl From<GraphKind> for SqlValue<'_> {
    fn from(value: GraphKind) -> Self {
        SqlValue::Str(std::borrow::Cow::Borrowed(value.as_static_str()))
    }
}

impl<'a> KDbRowBehavier<'a, GraphKind> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<GraphKind> {
        let s: String = self.try_get(key)?;

        GraphKind::try_from(s.as_str())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub struct GraphMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    pub archor: bool,

    #[gts_type = "Varchar<16>"]
    pub kind: GraphKind,

    pub content: Text,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl_otid_support! {GraphMeta}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub struct GraphData {
    #[gts_primary]
    pub sid: Varchar<100>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub content: Text,
}

impl_sid_support! {GraphData}

impl TryFrom<&GraphMeta> for GraphMetaEnum {
    type Error = anyhow::Error;

    fn try_from(value: &GraphMeta) -> Result<Self, Self::Error> {
        match value.kind {
            GraphKind::ExcalidrawV2 => Ok(GraphMetaEnum::ExcalidrawV2(serde_json::from_str(
                value.content.as_str(),
            )?)),
            GraphKind::MindElixirV1 => Ok(GraphMetaEnum::MindElixirV1(serde_json::from_str(
                value.content.as_str(),
            )?)),
            GraphKind::ExcalidrawLibraryV1 => Ok(GraphMetaEnum::ExcalidrawLibraryV1(
                serde_json::from_str(value.content.as_str())?,
            )),
        }
    }
}

impl GetKeys for GraphMetaEnum {
    fn get_keys(&self) -> Vec<String> {
        match self {
            GraphMetaEnum::ExcalidrawV2(d) => d.get_keys(),
            GraphMetaEnum::MindElixirV1(d) => d.keys.clone(),
            GraphMetaEnum::ExcalidrawLibraryV1(d) => vec![d.sid.clone()],
        }
    }
}
