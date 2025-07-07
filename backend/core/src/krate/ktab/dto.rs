use std::collections::HashMap;

use chin_sql::{str_type::Text, time_type::TID};
use chin_sql::SqlValue;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::decimal::Decimal;
use crate::model::omit_tid::OmitTID;

use super::{ *};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabMetaOverwriteReq {
    pub meta: KTabMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabMetaOverwriteRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabMetaQueryReq {
    pub table_id: TID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabMetaQueryRsp {
    pub meta: Option<KTabMeta>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabCellsOverwriteReq {
    pub table_id: TID,
    pub cells: Vec<KTabViewCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabCellsOverwriteRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KTabRowsQueryReqFilter {
    OneRowByIdx {
        row_tid: usize,
    },
    RowsByIdx {
        row_tid_included: usize,
        page_size: usize,
    },
    FieldSortPage {
        field_name: String,
        field_kind: KTabColumnStoreKind,
        start_included: usize,
        page_size: usize,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabRowsQueryReq {
    pub table_id: TID,
    pub filter: KTabRowsQueryReqFilter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabRowsQueryRspRow {
    pub row_tid: TID,
    pub cells: Vec<KTabViewCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabRowsQueryRsp {
    pub rows: Vec<KTabRowsQueryRspRow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KTabStoreValue {
    Decimal(Decimal),
    Text(Text),
    Date(DateTime<FixedOffset>),
    // Blob(Vec<u8>),
}

impl<'a> From<KTabStoreValue> for SqlValue<'a> {
    fn from(value: KTabStoreValue) -> Self {
        match value {
            KTabStoreValue::Decimal(v) => v.into(),
            KTabStoreValue::Text(v) => v.into(),
            KTabStoreValue::Date(v) => v.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabViewCell {
    pub row_tid: TID,
    pub column_name: String,
    pub value: KTabStoreValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabCell {
    pub tid: TID,
    pub table_otid: TID,
    pub col_otid: TID,
    pub row_otid: TID,
    pub omit_tid: OmitTID,
    pub cell_data: KTabStoreValue,
}

impl KTabCell {
    pub(super) fn into_view(self, column_names: &HashMap<TID, String>) -> Option<KTabViewCell> {
        let cell = KTabViewCell {
            row_tid: self.row_otid,
            column_name: column_names.get(&self.col_otid)?.to_string(),
            value: self.cell_data,
        };

        Some(cell)
    }
}

macro_rules! impl_from_ktab_cell {
    ($source:tt, $variant:ident) => {
        impl From<$source> for KTabCell {
            fn from(value: $source) -> Self {
                let $source {
                    table_otid,
                    col_otid,
                    row_otid,
                    omit_tid,
                    cell_data,
                    tid,
                } = value;

                Self {
                    tid,
                    table_otid,
                    col_otid,
                    row_otid,
                    omit_tid,
                    cell_data: KTabStoreValue::$variant(cell_data),
                }
            }
        }
    };
}
impl_from_ktab_cell! {KTabCellDate, Date}
impl_from_ktab_cell! {KTabCellText, Text}
impl_from_ktab_cell! {KTabCellDecimal, Decimal}

#[cfg(test)]
mod tests {
    use crate::krate::ktab::{
        KTabCellsOverwriteReq, KTabRowsQueryReq, KTabStoreValue, KTabViewCell,
    };

    #[test]
    fn test_key() {
        let req = KTabRowsQueryReq {
            table_id: 100.into(),
            filter: super::KTabRowsQueryReqFilter::FieldSortPage {
                field_name: "fn".to_owned(),
                field_kind: crate::krate::ktab::KTabColumnStoreKind::Date,
                start_included: 0,
                page_size: 100,
            },
        };

        println!("{:#?}", serde_json::to_string(&req).unwrap());
    }

    #[test]
    fn test_ktab_overwrite_cells_req() {
        let req = KTabCellsOverwriteReq {
            table_id: 123.into(),
            cells: vec![KTabViewCell {
                row_tid: 1.into(),
                column_name: "int".into(),
                value: KTabStoreValue::Decimal(123.into()),
            }],
        };

        println!("{:#?}", serde_json::to_string(&req).unwrap());
    }
}
