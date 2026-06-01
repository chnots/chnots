use chin_sql::GenerateTableSchema;
use chin_sql::SqlValue;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::enum_common_funcs;
use crate::impl_otid_support;
use crate::krate::toent::logic::todoevent::TodoPriorityEnum;
use crate::krate::toent::logic::todoevent::TodoStateEnum;
use crate::mapper::Curd;
use crate::mapper::db::KDbRow;
use crate::mapper::db::KDbRowBehavier;

#[derive(Debug, Clone, Sequence)]
pub enum ChnotKind {
    MarkdownWithToent,
    ExcalidrawV1,
    KFileV1,
    KTabV1,
    LLMChat,
    MindMapV1,
}

impl ChnotKind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            ChnotKind::MarkdownWithToent => "mdwt",
            ChnotKind::ExcalidrawV1 => "exdrv1",
            ChnotKind::KFileV1 => "resov1",
            ChnotKind::KTabV1 => "ktabv1",
            ChnotKind::LLMChat => "llm_chat",
            ChnotKind::MindMapV1 => "mindmapv1",
        }
    }
}
enum_common_funcs!(ChnotKind);

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

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Varchar<40>"]
    pub kind: ChnotKind,

    pub kspace: Varchar<40>,

    #[gts_type = "i64"]
    pub archive_tid: Option<TID>,

    #[gts_type = "i64"]
    pub pin_tid: Option<TID>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ChnotMeta {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            kind: value.try_get(Self::KIND)?,
            tid: value.try_get(Self::TID)?,
            kspace: value.try_get(Self::KSPACE)?,
            archive_tid: value.try_get(Self::ARCHIVE_TID)?,
            pin_tid: value.try_get(Self::PIN_TID)?,
        })
    }
}

impl Curd for ChnotMeta {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotMeta}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ChnotThreadOrder {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_primary]
    #[gts_key]
    #[gts_type = "i64"]
    pub thread_otid: TID,

    pub korder: i64,

    pub closed: bool,

    #[gts_type = "i32"]
    pub heading_level: i32,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ChnotThreadOrder {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            thread_otid: value.try_get(Self::THREAD_OTID)?,
            korder: value.try_get(Self::KORDER)?,
            tid: value.try_get(Self::TID)?,
            closed: value.try_get(Self::CLOSED)?,
            heading_level: value.try_get(Self::HEADING_LEVEL)?,
        })
    }
}

impl Curd for ChnotThreadOrder {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid, self.thread_otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ChnotThreadOrder}

fn todo_state_enum_to_sql(this: Option<TodoStateEnum>) -> Option<String> {
    this.map(|e| e.as_static_str().into())
}

fn todo_priority_enum_to_sql(this: Option<TodoPriorityEnum>) -> Option<i64> {
    this.map(|e| e.as_priority().into())
}
