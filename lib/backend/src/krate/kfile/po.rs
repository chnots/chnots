use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{GenerateTableSchema, str_type::Text};
use serde::{Deserialize, Serialize};

use crate::impl_otid_support;

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub struct KFileMeta {
    #[gts_primary]
    pub id: Varchar<100>,

    pub inline: bool,
    pub archor: bool,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub filename: Varchar<1024>,
    pub content_type: Varchar<200>,

    #[gts_type = "i64"]
    pub last_modified: TID,

    #[gts_key]
    pub sid: Varchar<100>,
    pub filesize: i64,
}

impl_otid_support! {KFileMeta}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub struct InlineKFile {
    #[gts_primary]
    pub sid: Varchar<100>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub content: Text,
}
