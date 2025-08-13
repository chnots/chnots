use chin_sql::SqlInserter;
use chin_tools::AResult;
use serde::{Serialize, de::DeserializeOwned};

use crate::mapper::{Curd, db::KDbRow};

pub mod decimal;
pub(crate) mod dto;
pub(crate) mod otid_table;

pub(crate) trait KSerde: Serialize + Send + Clone + DeserializeOwned + 'static {
    fn sql_inserter(&'_ self) -> SqlInserter<'_>;
    fn try_from_kdb_row(row: &KDbRow) -> AResult<Self>;
}

pub(crate) trait KOtidSupport: KSerde + Curd {
    fn get_otid_enum() -> otid_table::OtidTableEnum;
    fn table_name(hist: bool) -> &'static str;
    fn all_columns() -> &'static [&'static str];
}
