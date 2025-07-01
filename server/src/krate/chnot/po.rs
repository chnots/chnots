use chin_sql::str_type::Text;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::GenerateTableSchema;
use chin_sql::SqlValue;
/// Chnot: knot, which stands for the note.
///
/// Ancients used knots to record events,
/// so I use "knot" as the basic unit for my notebook,
/// but the name "knot" is too repetitive, so I made a change.
///
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use strum::Display;
use strum::IntoStaticStr;
use strum_macros::EnumString;

use crate::mapper::db::KDbRow;
use crate::mapper::db::KDbRowBehavier;
use crate::model::omit_tid::OmitTID;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    pub(crate) content: Text,
    pub(crate) archor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotMetadata {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,
    pub(crate) kspace: Varchar<40>,
    #[gts_type = "Varchar<40>"]
    pub(crate) kind: ChnotKind,
    pub(crate) pin_time: Option<DateTime<FixedOffset>>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    pub(crate) archive_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotTag {
    #[gts_primary]
    pub(crate) tag: Varchar<800>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    pub(crate) kspace: Varchar<40>,
    #[gts_type = "i64"]
    pub(crate) tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotKindRel {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,
    pub(crate) kind_id: Varchar<200>,
    #[gts_type = "i64"]
    pub(crate) tid: TID,
}



impl AsRef<str> for ChnotTag {
    fn as_ref(&self) -> &str {
        &self.tag.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, AsRefStr, IntoStaticStr)]
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

impl From<ChnotKind> for SqlValue<'_> {
    fn from(value: ChnotKind) -> Self {
        SqlValue::Str(std::borrow::Cow::Borrowed(value.into()))
    }
}

impl<'a> KDbRowBehavier<'a, ChnotKind> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<ChnotKind> {
        let s: String = self.try_get(key)?;

        Ok(ChnotKind::try_from(s.as_str())?)
    }
}
