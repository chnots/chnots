use std::{borrow::Cow, str::FromStr};

use chin_sql::{DbType, GenerateTableSql, SqlValue};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

use crate::mapper::db::{KDbRow, KDbRowBehavier};

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub struct Resource {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub workspace: String,
    #[gts_length = 200]
    pub content_type: String,

    #[gts_length = 512]
    pub ori_filename: String,
    pub filesize: i64,
    pub ori_last_modified: i64,

    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub struct InlineResource {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,

    #[gts_length = 40]
    pub rid: String,
    pub archor: bool,

    #[gts_length = 200]
    pub name: String,
    pub content: String,
    pub workspace: String,

    #[gts_length = 100]
    pub content_type: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Copy, Deserialize, EnumString, AsRefStr)]
pub enum KVType {
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
pub struct KTV {
    #[gts_primary]
    #[gts_length = 500]
    pub key: String,
    #[gts_primary]
    #[gts_length = 100]
    #[gts_type = "String"]
    pub ttype: KVType,
    pub value: String,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
