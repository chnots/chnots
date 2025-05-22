use std::{borrow::Cow, str::FromStr};
use strum::{AsRefStr, EnumString};

use crate::mapper::db::{KDbRow, KDbRowBehavier};

use chin_sql::{DbType, GenerateTableSql, SqlValue};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

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
