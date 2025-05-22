use core::str;
use std::{collections::HashMap, fmt::Display};

use chin_sql::{ChinSqlError, DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum KTabColumnType {
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
    Opt(Option<Box<KTabColumnType>>),
}

impl Display for KTabColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            KTabColumnType::Bool => "bool".into(),
            KTabColumnType::I8 => "i8".into(),
            KTabColumnType::I16 => "i16".into(),
            KTabColumnType::I32 => "i32".into(),
            KTabColumnType::I64 => "i64".into(),
            KTabColumnType::F64 => "f64".into(),
            KTabColumnType::Str => "str".into(),
            KTabColumnType::FixedOffset => "fixedoffset".into(),
            KTabColumnType::Utc => "utc".into(),
            KTabColumnType::Blob => "blob".into(),
            KTabColumnType::Opt(sql_value_type) => match sql_value_type {
                Some(v) => format!("opt.{}", v),
                None => unreachable!(),
            },
        };
        f.write_str(s.as_str())
    }
}

impl TryFrom<&str> for KTabColumnType {
    type Error = ChinSqlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = match value {
            "bool" => KTabColumnType::Bool,
            s => {
                if let Some(s) = s.strip_prefix("opt.") {
                    return KTabColumnType::try_from(s);
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
pub(crate) struct KTabColumnMeta {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) comment: String,
    pub(crate) stype: KTabColumnType,
}

#[derive(Clone, Debug, Serialize, Deserialize, GenerateTableSql)]
pub(crate) struct KTabMeta {
    pub(crate) id: String,
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

type_table!(KTabCellStr1024, String, #[gts_length = 1024]);
type_table!(KTabCellText, String);
type_table!(KTabCellInteger, i64);
type_table!(KTabCellDate, DateTime<FixedOffset>);
