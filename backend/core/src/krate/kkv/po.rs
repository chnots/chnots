use crate::{mapper::db::{KDbRow, KDbRowBehavier}, model::omit_tid::OmitTID};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum::{EnumString, IntoStaticStr};

use chin_sql::{str_type::{Text, Varchar}, GenerateTableSchema, SqlValue};

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, IntoStaticStr)]
pub(crate) enum KKVType {
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
pub(crate) struct KKV {
    #[gts_primary]
    pub(crate) key: Varchar<500>,
    #[gts_primary]
    #[gts_type = "Varchar<100>"]
    pub(crate) kind: KKVType,
    #[gts_primary]
    pub(crate) kspace: Varchar<40>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    pub(crate) value: Text,
}
