use chin_sql::SqlInserter;
use chin_tools::AResult;
use serde::{Serialize, de::DeserializeOwned};

use crate::mapper::db::KDbRow;

pub mod decimal;
pub(crate) mod dto;
pub(crate) mod otid_table;
pub(crate) mod sid_table;

pub use otid_table::*;
pub use sid_table::*;

pub(crate) trait KSerde: Serialize + Send + Clone + DeserializeOwned + 'static {
    fn sql_inserter(&'_ self) -> SqlInserter<'_>;
    fn try_from_kdb_row(row: &KDbRow) -> AResult<Self>;
}
