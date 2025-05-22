use serde::{Deserialize, Serialize};

use crate::model::db::ctable::{
    CTableCellDate, CTableCellInteger, CTableCellStr1024, CTableCellText, CTableMeta,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum CTableCell {
    String(CTableCellStr1024),
    Text(CTableCellText),
    Integer(CTableCellInteger),
    Date(CTableCellDate),
}

macro_rules! into_cell {
    ($sub_type:tt, $etype:tt) => {
        impl From<$sub_type> for CTableCell {
            fn from(value: $sub_type) -> Self {
                Self::$etype(value)
            }
        }
    };
}

into_cell! {CTableCellStr1024, String}
into_cell! {CTableCellText, Text}
into_cell! {CTableCellInteger, Integer}
into_cell! {CTableCellDate, Date}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteMetaReq {
    pub(crate) meta: CTableMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteMetaRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteRowReq {
    pub(crate) row: Vec<CTableCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteRowRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteCellReq {
    pub(crate) cell: CTableCell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableOverwriteCellRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryRowReq {
    pub(crate) table_id: String,
    pub(crate) row_index: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryRowRsp {
    pub(crate) row: Vec<CTableCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryTableMetaReq {
    pub(crate) table_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryTableMetaRsp {
    pub(crate) meta: CTableMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryTableDataReq {
    pub(crate) table_id: String,
    pub(crate) start_index: usize,
    pub(crate) page_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableQueryTableDataRsp {
    pub(crate) cells: Vec<CTableCell>,
}
