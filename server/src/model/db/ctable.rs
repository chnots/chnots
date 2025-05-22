use core::str;
use std::collections::HashMap;

use chin_sql::{ChinSqlError, DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum CTableColumnType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F64,
    Str,
    FixedOffset,
    Utc,
    Blob,
    Opt(Option<Box<CTableColumnType>>),
}

impl ToString for CTableColumnType {
    fn to_string(&self) -> String {
        match &self {
            CTableColumnType::Bool => "bool".into(),
            CTableColumnType::I8 => "i8".into(),
            CTableColumnType::I16 => "i16".into(),
            CTableColumnType::I32 => "i32".into(),
            CTableColumnType::I64 => "i64".into(),
            CTableColumnType::F64 => "f64".into(),
            CTableColumnType::Str => "str".into(),
            CTableColumnType::FixedOffset => "fixedoffset".into(),
            CTableColumnType::Utc => "utc".into(),
            CTableColumnType::Blob => "blob".into(),
            CTableColumnType::Opt(sql_value_type) => match sql_value_type {
                Some(v) => format!("opt.{}", v.to_string()),
                None => unreachable!(),
            },
        }
    }
}

impl TryFrom<&str> for CTableColumnType {
    type Error = ChinSqlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = match value {
            "bool" => CTableColumnType::Bool,
            s => {
                if s.starts_with("opt.") {
                    let s = &s[4..];
                    return CTableColumnType::try_from(s);
                } else {
                    Err(ChinSqlError::TransformError(format!(
                        "error getting sql value type, {}",
                        s
                    )))?
                }
            }
        };

        Ok(res)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CTableColumnMeta {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) comment: String,
    pub(crate) stype: CTableColumnType,
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSql)]
pub(crate) struct CTableMeta {
    pub(crate) id: String,
    #[gts_type = "String"]
    pub(crate) columns: HashMap<String, CTableColumnMeta>,
    pub(crate) table_name: String,
    pub(crate) table_comment: String,
    pub(crate) create_time: DateTime<FixedOffset>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) workspace: String,
    pub(crate) real_table: bool,
}

macro_rules! type_table {
    ($suffix:tt, $data_type:ty $(, #[$attr:meta])*) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSql)]
        pub(crate) struct $suffix {
            pub(crate) table_id: String,
            pub(crate) col_idx: i32,
            pub(crate) row_idx: i32,
            pub(crate) insert_time: DateTime<FixedOffset>,
            pub(crate) delete_time: Option<DateTime<FixedOffset>>,
            $(#[$attr])*
            pub(crate) cell_data: $data_type
        }
    }
}

type_table!(CTableCellStr1024, String, #[gts_length = 1024]);
type_table!(CTableCellText, String);
type_table!(CTableCellInteger, i64);
type_table!(CTableCellDate, DateTime<FixedOffset>);
