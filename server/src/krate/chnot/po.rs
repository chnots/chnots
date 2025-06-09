use chin_sql::DbType;
use chin_sql::GenerateTableSql;
use chin_sql::SqlValue;
/// Chnot: knot, which stands for the note.
///
/// Ancients used knots to record events,
/// so I use "knot" as the basic unit for my notebook,
/// but the name "knot" is too repetitive, so I made a change.
///
use chrono::{DateTime, FixedOffset};
use kdb_derives::KdbSqlInserter;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use strum::Display;
use strum_macros::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct ChnotRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) meta_id: String,
    pub(crate) content: String,
    pub(crate) omit_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct ChnotMetadata {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) kspace: String,
    #[gts_length = 40]
    pub(crate) kind: String,
    pub(crate) pin_time: Option<DateTime<FixedOffset>>,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
    pub(crate) archive_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct ChnotTag {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) kspace: String,
    #[gts_length = 800]
    pub(crate) tag: String,
    #[gts_type = "i32"]
    pub(crate) category: ChnotTagType,
    pub(crate) chnot_meta_id: String,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

impl AsRef<str> for ChnotTag {
    fn as_ref(&self) -> &str {
        &self.tag
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, AsRefStr)]
pub(crate) enum ChnotKind {
    #[strum(serialize = "mdwt")]
    #[serde(rename = "mdwt")]
    MarkdownWithToent,

    #[strum(serialize = "exdrv1")]
    #[serde(rename = "exdrv1")]
    ExcalidrawV1,

    #[strum(serialize = "resov1")]
    #[serde(rename = "resov1")]
    KFileV1,

    #[strum(serialize = "ktabv1")]
    #[serde(rename = "ktabv1")]
    KTabV1,

    #[strum(serialize = "llm_chat")]
    #[serde(rename = "llm_chat")]
    LLMChat,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub(crate) enum ChnotTagType {
    Dir = 99,
    ParentDir = 98,
    Common = 1,
}

impl From<ChnotTagType> for SqlValue<'_> {
    fn from(value: ChnotTagType) -> Self {
        SqlValue::I32(value as i32)
    }
}
