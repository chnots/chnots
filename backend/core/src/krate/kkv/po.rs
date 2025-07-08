use crate::{
    mapper::db::{KDbRow, KDbRowBehavier},
    model::omit_tid::OmitTID,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum::{EnumString, IntoStaticStr};

use chin_sql::{
    GenerateTableSchema, SqlValue,
    str_type::{Text, Varchar},
    time_type::TID,
};

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, IntoStaticStr)]
pub enum KKVType {
    #[strum(serialize = "k_space_info")]
    #[serde(rename = "k_space_info")]
    KSpaceInfo,
    #[strum(serialize = "def")]
    #[serde(rename = "def")]
    Default,
}

impl<'a> From<KKVType> for SqlValue<'a> {
    fn from(val: KKVType) -> Self {
        let s: &'static str = val.into();
        SqlValue::Str(Cow::Borrowed(s))
    }
}

impl<'a> KDbRowBehavier<'a, KKVType> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<KKVType> {
        let s: String = self.try_get(key)?;

        Ok(KKVType::try_from(s.as_str())?)
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
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub archor: bool,

    pub value: Text,
}

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
