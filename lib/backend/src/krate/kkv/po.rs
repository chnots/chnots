use crate::{
    enum_common_funcs, impl_otid_support,
    mapper::db::{KDbRow, KDbRowBehavier},
};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use chin_sql::{
    GenerateTableSchema, SqlValue,
    str_type::{Text, Varchar},
    time_type::TID,
};

#[derive(Debug, Clone, Copy, Sequence)]
pub enum KKVType {
    KSpaceInfo,
    Default,
}

impl KKVType {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            KKVType::KSpaceInfo => "k_space_info",
            KKVType::Default => "def",
        }
    }
}

enum_common_funcs!(KKVType);

impl<'a> From<KKVType> for SqlValue<'a> {
    fn from(val: KKVType) -> Self {
        let s: &'static str = val.as_static_str();
        SqlValue::Str(Cow::Borrowed(s))
    }
}

impl<'a> KDbRowBehavier<'a, KKVType> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<KKVType> {
        let s: String = self.try_get(key)?;

        KKVType::try_from(s.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
#[allow(clippy::upper_case_acronyms)]
pub struct KKV {
    #[gts_primary]
    pub key: Varchar<500>,
    #[gts_primary]
    pub kind: Varchar<100>,
    #[gts_primary]
    pub kspace: Varchar<40>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub archor: bool,

    pub value: Text,
}

impl_otid_support! {KKV}

/// only for cache, we do not sync this.
#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
#[allow(clippy::upper_case_acronyms)]
pub struct KKVTransient {
    #[gts_primary]
    pub key: Varchar<500>,
    pub value: Text,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}
