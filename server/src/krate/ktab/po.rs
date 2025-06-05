use core::str;
use std::collections::HashMap;

use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use kdb_derives::KdbSqlInserter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum KTabColumnStoreKind {
    I64,
    F64,
    Str,
    Date,
    // Blob,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabColumnMeta {
    pub(crate) idx: i64,
    pub(crate) name: String,
    pub(crate) comment: String,
    pub(crate) store_kind: KTabColumnStoreKind,
    pub(crate) view_kind: String,
    pub(crate) required: bool,
    pub(crate) order_by: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSql)]
pub(crate) struct KTabMeta {
    #[gts_primary]
    pub(crate) id: i64,
    #[gts_type = "String"]
    pub(crate) columns: HashMap<String, KTabColumnMeta>,
    pub(crate) table_name: String,
    pub(crate) table_comment: String,
    pub(crate) create_time: DateTime<FixedOffset>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) kspace: String,
    pub(crate) real_table: bool,
}

macro_rules! type_table {
    ($sname:tt, $data_type:ty $(, #[$attr:meta])*) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSql, KdbSqlInserter)]
        pub(crate) struct $sname {
            pub(crate) table_id: i64,
            pub(crate) col_idx: i64, // actually is the insert time(unix timestamp), so it is easy for data merge
            pub(crate) row_idx: i64, // actually is the insert time(unix timestamp), so it is easy for data merge
            pub(crate) insert_time: DateTime<FixedOffset>,
            pub(crate) delete_time: Option<DateTime<FixedOffset>>,
            $(#[$attr])*
            pub(crate) cell_data: $data_type
        }
    }
}

type_table!(KTabCellText, String);
type_table!(KTabCellI64, i64);
type_table!(KTabCellDate, DateTime<FixedOffset>);
type_table!(KTabCellF64, f64);

#[cfg(test)]
mod ktab_column_type_test {
    #[test]
    fn test() {
        use crate::krate::ktab::po::KTabColumnStoreKind;

        println!("{:?}", serde_json::to_string(&KTabColumnStoreKind::I64));
    }
}
