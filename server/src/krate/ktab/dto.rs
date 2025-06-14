use std::collections::HashMap;

use chin_sql::SqlValue;
use chin_tools::time_type::TID;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

use super::{decimal::Decimal, *};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabMetaOverwriteReq {
    pub(crate) meta: KTabMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabMetaOverwriteRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabMetaQueryReq {
    pub(crate) table_id: TID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabMetaQueryRsp {
    pub(crate) meta: Option<KTabMeta>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabCellsOverwriteReq {
    pub(crate) table_id: TID,
    pub(crate) cells: Vec<KTabViewCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabCellsOverwriteRsp {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum KTabRowsQueryReqFilter {
    OneRowByIdx {
        row_idx: usize,
    },
    RowsByIdx {
        row_idx_included: usize,
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
pub(crate) struct KTabRowsQueryReq {
    pub(crate) table_id: TID,
    pub(crate) filter: KTabRowsQueryReqFilter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabRowsQueryRspRow {
    pub(crate) row_idx: TID,
    pub(crate) cells: Vec<KTabViewCell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabRowsQueryRsp {
    pub(crate) rows: Vec<KTabRowsQueryRspRow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum KTabStoreValue {
    Decimal(Decimal),
    Text(String),
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
pub(crate) struct KTabViewCell {
    pub(crate) row_idx: TID,
    pub(crate) column_name: String,
    pub(crate) value: KTabStoreValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabCell {
    pub(crate) tid: TID,
    pub(crate) table_id: TID,
    pub(crate) col_idx: TID,
    pub(crate) row_idx: TID,
    pub(crate) omit_tid: OmitTID,
    pub(crate) cell_data: KTabStoreValue,
}

impl KTabCell {
    pub(super) fn into_view(self, column_names: &HashMap<TID, String>) -> Option<KTabViewCell> {
        let cell = KTabViewCell {
            row_idx: self.row_idx,
            column_name: column_names.get(&self.col_idx)?.to_string(),
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
                    table_id,
                    col_idx,
                    row_idx,
                    omit_tid,
                    cell_data,
                    tid
                } = value;

                Self {
                    tid,
                    table_id,
                    col_idx,
                    row_idx,
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
        KTabCellsOverwriteReq, KTabStoreValue, KTabRowsQueryReq, KTabViewCell,
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
                row_idx: 1.into(),
                column_name: "int".into(),
                value: KTabStoreValue::Decimal(123.into()),
            }],
        };

        println!("{:#?}", serde_json::to_string(&req).unwrap());
    }
}
