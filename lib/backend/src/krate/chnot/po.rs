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
pub struct ChnotRecord {
    #[gts_primary]
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

impl Curd for ChnotRecord {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.meta_otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotRecord}

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
    #[gts_type = "Varchar<40>"]
    pub kind: ChnotKind,
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

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct ChnotKindRel {
    #[gts_primary]
    #[gts_type = "i64"]
    pub meta_otid: TID,
    #[gts_key]
    pub kind_id: Varchar<200>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl Curd for ChnotKindRel {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.meta_otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotKindRel}

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

#[derive(Debug, Clone, Sequence, Copy)]
pub(crate) enum ChnotBlockEnum {
    Heading,
    ListItem,
}

impl ChnotBlockEnum {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            ChnotBlockEnum::Heading => "HEADING",
            ChnotBlockEnum::ListItem => "LISTITEM",
        }
    }
}

fn chnot_block_enum_to_sql(this: ChnotBlockEnum) -> String {
    this.as_static_str().to_string()
}

enum_common_funcs!(ChnotBlockEnum);

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotBlock {
    #[gts_primary]
    pub id: Varchar<100>,
    #[gts_type = "i64"]
    pub chnot_otid: TID,
    pub title: Varchar<500>,
    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "chnot_block_enum_to_sql"]
    pub kind: ChnotBlockEnum,
    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: TodoStateEnum,
    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: TodoPriorityEnum,
    pub closed: bool,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ChnotBlock {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let cb = Self {
            id: value.try_get(Self::ID)?,
            chnot_otid: value.try_get(Self::CHNOT_OTID)?,
            title: value.try_get(Self::TITLE)?,
            kind: {
                let s: String = value.try_get(Self::KIND)?;
                ChnotBlockEnum::try_from(s.as_str())?
            },
            tid: value.try_get(Self::TID)?,
            todo_state: {
                let s: String = value.try_get(Self::TODO_STATE)?;
                TodoStateEnum::try_from(s.as_str())?
            },
            todo_priority: {
                let s: i64 = value.try_get(Self::TODO_PRIORITY)?;
                TodoPriorityEnum::try_from(s)?
            },
            closed: value.try_get(Self::CLOSED)?,
        };

        Ok(cb)
    }
}

impl Curd for ChnotBlock {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.id.clone())
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotBlock}

fn todo_state_enum_to_sql(this: TodoStateEnum) -> String {
    this.as_static_str().to_string()
}

fn todo_priority_enum_to_sql(this: TodoPriorityEnum) -> i64 {
    this.as_priority().into()
}
