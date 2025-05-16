use serde::{Deserialize, Serialize};

use crate::model::db::ctable::{
    CTableCellDate, CTableCellInteger, CTableCellStr1024, CTableCellText, CTableMeta,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CTableCell {
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
pub struct CTableOverwriteMetaReq {
    pub meta: CTableMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableOverwriteMetaRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableOverwriteRowReq {
    pub row: Vec<CTableCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableOverwriteRowRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableOverwriteCellReq {
    pub cell: CTableCell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableOverwriteCellRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryRowReq {
    pub table_id: String,
    pub row_index: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryRowRsp {
    pub row: Vec<CTableCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryTableMetaReq {
    pub table_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryTableMetaRsp {
    pub meta: CTableMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryTableDataReq {
    pub table_id: String,
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CTableQueryTableDataRsp {
    pub cells: Vec<CTableCell>,
}
