use chin_sql::{str_type::Text, GenerateTableSchema};
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub(crate) struct KFileMeta {
    #[gts_primary]
    pub(crate) id: Varchar<100>,

    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    pub(crate) inline: bool,
    pub(crate) archor: bool,

    #[gts_unique]
    #[gts_type = "i64"]
    pub(crate) tid: TID,

    pub(crate) filename: Varchar<1024>,
    pub(crate) content_type: Varchar<200>,

    #[gts_type = "i64"]
    pub(crate) last_modified: TID,

    #[gts_key]
    pub(crate) sid: Varchar<100>,
    pub(crate) filesize: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema)]
pub(crate) struct InlineKFile {
    #[gts_primary]
    pub(crate) sid: Varchar<100>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub(crate) tid: TID,

    pub(crate) content: Text,
}
