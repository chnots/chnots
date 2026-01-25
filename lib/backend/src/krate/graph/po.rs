use std::collections::BTreeMap;
use std::str::FromStr;

use chin_sql::SqlValue;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{GenerateTableSchema, str_type::Text};
use chin_tools::AResult;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mapper::db::{KDbRow, KDbRowBehavier};
use crate::{enum_common_funcs, impl_otid_support};

pub trait GetKeys {
    fn get_keys(&self) -> Vec<String>;
}

#[derive(Debug, Clone, Sequence)]
pub enum GraphKind {
    ExcalidrawV2,
    MindElixirV1,
}

impl GraphKind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            GraphKind::ExcalidrawV2 => "exdrv2",
            GraphKind::MindElixirV1 => "mielixirv1",
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
