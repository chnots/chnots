use chin_sql::GenerateTableSchema;
use chin_sql::SqlValue;
use chin_sql::str_type::Text;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use serde::{Deserialize, Serialize};

use crate::impl_otid_support;
use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::mapper::Curd;

fn opt_todo_tosql<'a>(opt: Option<TodoEvent>) -> SqlValue<'a> {
    match opt {
        Some(te) => te.into(),
        None => SqlValue::Null(chin_sql::LogicFieldType::Varchar(20)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct MdwtRecord {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
    #[gts_type = "Varchar<20>"]
    #[gts_tosql = "opt_todo_tosql"]
    pub todo_event: Option<TodoEvent>,
    pub content: Text,
    pub archor: bool,
}

impl Curd for MdwtRecord {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {MdwtRecord}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub struct MdwtTag {
    #[gts_primary]
    pub tag: Varchar<800>,
    #[gts_primary]
    #[gts_type = "i64"]
    pub mdwt_otid: TID,
    pub kspace: Varchar<40>,
    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl Curd for MdwtTag {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.tag.clone(), self.mdwt_otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {MdwtTag}

impl AsRef<str> for MdwtTag {
    fn as_ref(&self) -> &str {
        self.tag.as_str()
    }
}
