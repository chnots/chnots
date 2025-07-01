use super::decimal::Decimal;
use std::collections::HashMap;

use chin_sql::str_type::Text;
use chin_sql::GenerateTableSchema;
use chin_sql::{str_type::Varchar, time_type::TID};
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

fn map_to_sql(value: HashMap<String, KTabColumnMeta>) -> String {
    // Only serde_json do not fail.
    serde_json::to_string(&value).unwrap()
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct KTabMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) tid: TID,

    #[gts_primary]
    #[gts_type = "i64"]
    pub(crate) omit_tid: OmitTID,

    #[gts_type = "Text"]
    #[gts_tosql = "map_to_sql"]
    pub(crate) columns: HashMap<String, KTabColumnMeta>,
    pub(crate) table_name: Varchar<300>,
    pub(crate) table_comment: Varchar<1000>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) real_table: bool,
}

macro_rules! type_table {
    ($sname:tt, $data_type:ty $(, #[$attr:meta])*) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema)]
        pub(crate) struct $sname {
            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) table_id: TID,

            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) col_tid: TID, // actually is the insert time(unix timestamp), so it is easy for data merge

            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) row_tid: TID, // actually is the insert time(unix timestamp), so it is easy for data merge

            #[gts_primary]
            #[gts_type = "i64"]
            pub(crate) omit_tid: OmitTID,

            #[gts_type = "i64"]
            pub(crate) tid: TID,

            $(#[$attr])*
            pub(crate) cell_data: $data_type
        }
    }
}

type_table!(KTabCellText, Text);
type_table!(KTabCellDecimal, Decimal, #[gts_type = "Text"]);
type_table!(KTabCellDate, DateTime<FixedOffset>);

#[cfg(test)]
mod ktab_column_type_test {
    #[test]
    fn test() {
        use crate::krate::ktab::po::KTabColumnStoreKind;

        println!("{:?}", serde_json::to_string(&KTabColumnStoreKind::Str));
    }
}
