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
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::enum_common_funcs;
use crate::impl_otid_support;
use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::krate::toent::logic::todoevent::TodoPriorityEnum;
use crate::krate::toent::logic::todoevent::TodoStateEnum;
use crate::mapper::Curd;
use crate::mapper::db::KDbRow;
use crate::mapper::db::KDbRowBehavier;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotBlockRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub block_otid: TID,
    #[gts_type = "i64"]
    pub meta_otid: TID,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_type = "Varchar<20>"]
    #[gts_tosql = "opt_todo_tosql"]
    pub todo_event: Option<TodoEvent>,
    pub content: Text,
    pub archor: bool,
}

impl Curd for ChnotBlockRecord {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.block_otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotBlockRecord}

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
    pub otid: TID,
    pub kspace: Varchar<40>,
    pub pin_time: Option<DateTime<FixedOffset>>,
    pub archive_time: Option<DateTime<FixedOffset>>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl Curd for ChnotMetadata {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
impl_otid_support! {ChnotMetadata}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotTag {
    #[gts_primary]
    pub tag: Varchar<800>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub meta_otid: TID,
    pub kspace: Varchar<40>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl Curd for ChnotTag {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.tag.clone(), self.meta_otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotTag}

impl AsRef<str> for ChnotTag {
    fn as_ref(&self) -> &str {
        self.tag.as_str()
    }
}

#[derive(Debug, Clone, Sequence)]
pub enum ChnotKind {
    MarkdownWithToent,
    ExcalidrawV1,
    KFileV1,
    KTabV1,
    LLMChat,
}

impl ChnotKind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            ChnotKind::MarkdownWithToent => "mdwt",
            ChnotKind::ExcalidrawV1 => "exdrv1",
            ChnotKind::KFileV1 => "resov1",
            ChnotKind::KTabV1 => "ktabv1",
            ChnotKind::LLMChat => "llm_chat",
        }
    }
}
enum_common_funcs!(ChnotKind);

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        SqlValue::Str(std::borrow::Cow::Borrowed(value.as_static_str()))
    }
}

impl<'a> KDbRowBehavier<'a, ChnotKind> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<ChnotKind> {
        let s: String = self.try_get(key)?;

        ChnotKind::try_from(s.as_str())
    }
}

fn todo_state_enum_to_sql(this: Option<TodoStateEnum>) -> Option<String> {
    this.map(|e| e.as_static_str().into())
}

fn todo_priority_enum_to_sql(this: Option<TodoPriorityEnum>) -> Option<i64> {
    this.map(|e| e.as_priority().into())
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotBlockMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,
    #[gts_key]
    #[gts_type = "i64"]
    pub chnot_otid: TID,

    #[gts_type = "Varchar<40>"]
    pub kind: ChnotKind,
    #[gts_key]
    pub kind_id: Varchar<200>,

    pub korder: i64,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ChnotBlockMeta {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            chnot_otid: value.try_get(Self::CHNOT_OTID)?,
            kind: value.try_get(Self::KIND)?,
            kind_id: value.try_get(Self::KIND_ID)?,
            korder: value.try_get(Self::KORDER)?,
            tid: value.try_get(Self::TID)?,
        })
    }
}

impl Curd for ChnotBlockMeta {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotBlockMeta}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotBlockToent {
    #[gts_primary]
    #[gts_type = "i64"]
    pub block_otid: TID,
    #[gts_key]
    #[gts_type = "i64"]
    pub chnot_otid: TID,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    pub todo_closed: bool,

    pub note: Option<Text>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ChnotBlockToent {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            block_otid: value.try_get(Self::BLOCK_OTID)?,
            chnot_otid: value.try_get(Self::CHNOT_OTID)?,
            tid: value.try_get(Self::TID)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
            todo_closed: value.try_get(Self::TODO_CLOSED)?,
            note: value.try_get(Self::NOTE)?,
        })
    }
}

impl Curd for ChnotBlockToent {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.block_otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotBlockToent}
