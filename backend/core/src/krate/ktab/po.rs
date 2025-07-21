use std::collections::HashMap;

use chin_sql::str_type::Text;
use chin_sql::GenerateTableSchema;
use chin_sql::{str_type::Varchar, time_type::TID};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::decimal::Decimal;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KTabColumnStoreKind {
    Decimal,
    Str,
    Date,
    // Blob,
}

pub type KTabColumnViewKind = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KTabColumnMeta {
    pub idx: TID,
    pub name: String,
    pub comment: String,
    pub store_kind: KTabColumnStoreKind,
    pub view_kind: KTabColumnViewKind,
    pub required: bool,
    pub order_by: i32,
}

fn map_to_sql(value: HashMap<String, KTabColumnMeta>) -> String {
    // Only serde_json do not fail.
    serde_json::to_string(&value).unwrap()
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema)]
pub struct KTabMeta {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Text"]
    #[gts_tosql = "map_to_sql"]
    pub columns: HashMap<String, KTabColumnMeta>,
    pub table_name: Varchar<300>,
    pub table_comment: Varchar<1000>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub real_table: bool,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

macro_rules! type_table {
    ($sname:tt, $data_type:ty $(, #[$attr:meta])*) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSchema)]
        pub struct $sname {
            #[gts_primary]
            #[gts_type = "i64"]
            pub table_otid: TID,

            #[gts_primary]
            #[gts_type = "i64"]
            pub col_otid: TID, // actually is the insert time(unix timestamp), so it is easy for data merge

            #[gts_primary]
            #[gts_type = "i64"]
            pub row_otid: TID, // actually is the insert time(unix timestamp), so it is easy for data merge

            #[gts_unique]
            #[gts_type = "i64"]
            pub tid: TID,

            $(#[$attr])*
            pub cell_data: $data_type
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
