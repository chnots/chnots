use crate::mapper::db::{KDbRow, KDbRowBehavier};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};
use strum::{AsRefStr, EnumString};

use chin_sql::{ChinSqlCrud, GenerateTableSchema, SqlValue};

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, AsRefStr)]
pub(crate) enum KKVType {
    #[strum(serialize = "chnot_sub_type")]
    #[serde(rename = "chnot_sub_type")]
    ChnotSubType,
    #[strum(serialize = "k_space_info")]
    #[serde(rename = "k_space_info")]
    KSpaceInfo,
    #[strum(serialize = "to_kfile")]
    #[serde(rename = "to_kfile")]
    ToKFile,
    #[strum(serialize = "def")]
    #[serde(rename = "def")]
    Default,
}

impl<'a> From<KKVType> for SqlValue<'a> {
    fn from(val: KKVType) -> Self {
        SqlValue::Str(Cow::Owned(val.as_ref().to_owned()))
    }
}

impl KDbRowBehavier<KKVType> for KDbRow {
    fn try_get(&self, key: &str) -> chin_tools::AResult<KKVType> {
        let s: String = self.try_get(key)?;
        Ok(KKVType::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct KKV {
    #[gts_primary]
    #[gts_length = 500]
    pub(crate) key: String,
    #[gts_primary]
    #[gts_length = 100]
    #[gts_type = "String"]
    pub(crate) kind: KKVType,
    #[gts_primary]
    #[gts_length = 100]
    pub(crate) kspace: String,
    pub(crate) value: String,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}
