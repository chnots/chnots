use std::{borrow::Borrow, ops::Deref};

use anyhow::anyhow;
use chin_sql::SqlValueRow;
use chin_sql::{SqlValue, SqlValueOwned};
use chin_sql::time_type::TID;
use chin_tools::AResult;
use chrono::{DateTime, FixedOffset};

use super::KDbRowBehavier;

macro_rules! row_behavier {
    ($tp:ty) => {
        impl KDbRowBehavier<$tp> for SqlValueRow<SqlValueOwned> {
            fn try_get(&self, key: &str) -> AResult<$tp> {
                match self.row.get(key) {
                    Some(value) => {
                        let s: &SqlValue<'static> = value.borrow();
                        Ok(<$tp>::try_from(s.clone())?)
                    }
                    None => Err(anyhow!("absent value for key: {}", key)),
                }
            }
        }

        impl KDbRowBehavier<Option<$tp>> for SqlValueRow<SqlValueOwned> {
            fn try_get(&self, key: &str) -> AResult<Option<$tp>> {
                match self.row.get(key) {
                    Some(value) => match value.deref() {
                        SqlValue::Null(_) => Ok(None),
                        v => Ok(Some(<$tp>::try_from(v.borrow().clone())?)),
                    },
                    None => Err(anyhow!("absent value for key: {}", key)),
                }
            }
        }
    };
}

row_behavier! {i64}
row_behavier! {f64}
row_behavier! {i32}
row_behavier! {String}
row_behavier! {DateTime<FixedOffset>}
row_behavier! {bool}
row_behavier! {TID}