use chin_sql::GenerateTableSchema;
use chin_sql::SqlValue;
use chin_sql::str_type::Text;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
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

use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::mapper::db::KDbRow;
use crate::mapper::db::KDbRowBehavier;
use crate::model::omit_tid::OmitTID;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_type = "Varchar<20>"]
    #[gts_tosql = "opt_todo_tosql"]
    pub todo_event: Option<TodoEvent>,
    pub content: Text,
    pub archor: bool,
}

fn opt_todo_tosql<'a>(opt: Option<TodoEvent>) -> SqlValue<'a> {
    match opt {
        Some(te) => te.into(),
        None => SqlValue::Null(chin_sql::LogicFieldType::Varchar(20)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotMetadata {
    #[gts_primary]
    #[gts_type = "i64"]
    pub tid: TID,
    pub kspace: Varchar<40>,
    #[gts_type = "Varchar<40>"]
    pub kind: ChnotKind,
    pub pin_time: Option<DateTime<FixedOffset>>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    pub archive_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotTag {
    #[gts_primary]
    pub tag: Varchar<800>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    pub kspace: Varchar<40>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotKindRel {
    #[gts_primary]
    #[gts_type = "i64"]
    pub meta_tid: TID,
    #[gts_primary]
    #[gts_type = "i64"]
    pub omit_tid: OmitTID,
    #[gts_key]
    pub kind_id: Varchar<200>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl AsRef<str> for ChnotTag {
    fn as_ref(&self) -> &str {
        self.tag.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, AsRefStr, IntoStaticStr)]
pub enum ChnotKind {
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
pub enum ChnotTagType {
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
