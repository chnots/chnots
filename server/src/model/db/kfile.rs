use std::{borrow::Cow, str::FromStr};

use chin_sql::{DbType, GenerateTableSql, SqlValue};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

use crate::mapper::db::{KDbRow, KDbRowBehavier};

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub(crate) struct KFile {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) workspace: String,
    #[gts_length = 200]
    pub(crate) content_type: String,

    #[gts_length = 512]
    pub(crate) ori_filename: String,
    pub(crate) filesize: i64,
    pub(crate) ori_last_modified: i64,

    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub(crate) struct InlineKFile {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,

    #[gts_length = 40]
    pub(crate) rid: String,
    pub(crate) archor: bool,

    #[gts_length = 200]
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) workspace: String,

    #[gts_length = 100]
    pub(crate) content_type: String,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, AsRefStr)]
pub(crate) enum KVType {
    #[strum(serialize = "chnot_sub_type")]
    #[serde(rename = "chnot_sub_type")]
    ChnotSubType,
    #[strum(serialize = "def")]
    #[serde(rename = "def")]
    Default,
}

impl<'a> Into<SqlValue<'a>> for KVType {
    fn into(self) -> SqlValue<'a> {
        SqlValue::Str(Cow::Owned(self.as_ref().to_owned()))
    }
}

impl<'a> KDbRowBehavier<'a, KVType> for KDbRow<'a> {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<KVType> {
        let s: String = self.try_get(key)?;
        Ok(KVType::from_str(&s)?)
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
    pub(crate) ttype: KVType,
    pub(crate) value: String,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}
