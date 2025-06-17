use super::decimal::Decimal;
use chin_sql::ChinSqlCrud;
use std::collections::HashMap;

use chin_sql::time_type::TID;
use chin_sql::GenerateTableSchema;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omit_tid::OmitTID;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum KTabColumnStoreKind {
    Decimal,
    Str,
    Date,
    // Blob,
}

pub(crate) type KTabColumnViewKind = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KTabColumnMeta {
    pub(crate) idx: TID,
    pub(crate) name: String,
    pub(crate) comment: String,
    pub(crate) store_kind: KTabColumnStoreKind,
    pub(crate) view_kind: KTabColumnViewKind,
    pub(crate) required: bool,
    pub(crate) order_by: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct KTabMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,

    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    #[gts_type = "String"]
    pub(crate) columns: HashMap<String, KTabColumnMeta>,
    pub(crate) table_name: String,
    pub(crate) table_comment: String,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) real_table: bool,
}

macro_rules! type_table {
    ($sname:tt, $data_type:ty $(, #[$attr:meta])*) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema, ChinSqlCrud)]
        pub(crate) struct $sname {
            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) tid: TID,

            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) omit_tid: OmitTID,

            #[gts_type = "i64"]
            pub(crate) table_id: TID,
            #[gts_type = "i64"]
            pub(crate) col_idx: TID, // actually is the insert time(unix timestamp), so it is easy for data merge
            #[gts_type = "i64"]
            pub(crate) row_idx: TID, // actually is the insert time(unix timestamp), so it is easy for data merge

            $(#[$attr])*
            pub(crate) cell_data: $data_type
        }
    }
}

type_table!(KTabCellText, String);
type_table!(KTabCellDecimal, Decimal, #[gts_type = "String"]);
type_table!(KTabCellDate, DateTime<FixedOffset>);

#[cfg(test)]
mod ktab_column_type_test {
    #[test]
    fn test() {
        use crate::krate::ktab::po::KTabColumnStoreKind;

        println!("{:?}", serde_json::to_string(&KTabColumnStoreKind::Str));
    }
}
