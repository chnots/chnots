use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum KTabCell {
    String(KTabCellStr1024),
    Text(KTabCellText),
    Integer(KTabCellInteger),
    Date(KTabCellDate),
}

macro_rules! into_cell {
    ($sub_type:tt, $etype:tt) => {
        impl From<$sub_type> for KTabCell {
            fn from(value: $sub_type) -> Self {
                Self::$etype(value)
            }
        }
    };
}

into_cell! {KTabCellStr1024, String}
into_cell! {KTabCellText, Text}
into_cell! {KTabCellInteger, Integer}
into_cell! {KTabCellDate, Date}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteMetaReq {
    pub(crate) meta: KTabMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteMetaRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteRowReq {
    pub(crate) row: Vec<KTabCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteRowRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteCellReq {
    pub(crate) cell: KTabCell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabOverwriteCellRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryRowReq {
    pub(crate) table_id: String,
    pub(crate) row_index: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryRowRsp {
    pub(crate) row: Vec<KTabCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryTableMetaReq {
    pub(crate) table_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryTableMetaRsp {
    pub(crate) meta: KTabMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryTableDataReq {
    pub(crate) table_id: String,
    pub(crate) start_index: usize,
    pub(crate) page_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabQueryTableDataRsp {
    pub(crate) cells: Vec<KTabCell>,
}
