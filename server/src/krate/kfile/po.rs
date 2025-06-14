use chin_sql::time_type::TID;
use chin_sql::{ChinSqlCrud, GenerateTableSchema};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct KFile {
    #[gts_primary]
    pub(crate) sid: String,

    #[gts_type = "i64"]
    pub(crate) tid: TID,

    #[gts_length = 200]
    pub(crate) content_type: String,

    #[gts_length = 1024]
    pub(crate) ori_filename: String,
    pub(crate) ori_last_modified: i64,
    pub(crate) filesize: i64,

    #[gts_type = "i64"]
    #[serde(default = "OmitTID::never")]
    pub(crate) omit_tid: OmitTID,
    #[gts_length = 40]
    pub(crate) kspace: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSchema, ChinSqlCrud)]
pub(crate) struct InlineKFile {
    #[gts_primary]
    pub(crate) sid: String,

    #[gts_type = "i64"]
    pub(crate) tid: TID,

    #[gts_length = 40]
    pub(crate) kspace: String,
    #[gts_length = 100]
    pub(crate) content_type: String,

    #[gts_length = 1024]
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) archor: bool,

    #[gts_type = "i64"]
    #[serde(default = "OmitTID::never")]
    pub(crate) omit_tid: OmitTID,
}
