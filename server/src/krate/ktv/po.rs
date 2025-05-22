use crate::mapper::db::{KDbRow, KDbRowBehavier};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};
use strum::{AsRefStr, EnumString};

use chin_sql::{DbType, GenerateTableSql, SqlValue};

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, AsRefStr)]
pub(crate) enum KTVType {
    #[strum(serialize = "chnot_sub_type")]
    #[serde(rename = "chnot_sub_type")]
    ChnotSubType,
    #[strum(serialize = "def")]
    #[serde(rename = "def")]
    Default,
}

impl<'a> From<KTVType> for SqlValue<'a> {
    fn from(val: KTVType) -> Self {
        SqlValue::Str(Cow::Owned(val.as_ref().to_owned()))
    }
}

impl<'a> KDbRowBehavier<'a, KTVType> for KDbRow<'a> {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<KTVType> {
        let s: String = self.try_get(key)?;
        Ok(KTVType::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub(crate) struct KTV {
    #[gts_primary]
    #[gts_length = 500]
    pub(crate) key: String,
    #[gts_primary]
    #[gts_length = 100]
    #[gts_type = "String"]
    pub(crate) ttype: KTVType,
    pub(crate) value: String,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}
