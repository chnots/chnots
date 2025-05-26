use std::{borrow::Borrow, ops::Deref};

use anyhow::anyhow;
use chin_sql::{SqlValue, SqlValueOwned};
use chin_tools::AResult;
use chrono::{DateTime, FixedOffset};
use chin_sql::{SqlValueRow};

use super::KDbRowBehavier;

macro_rules! row_behavier {
    ($tp:ty) => {
        impl<'a, 'b> KDbRowBehavier<'b, $tp> for SqlValueRow<SqlValueOwned> {
            fn try_get(&'b self, key: &str) -> AResult<$tp> {
                match self.row.get(key) {
                    Some(value) => {
                        let s: &SqlValue<'static> = value.borrow();
                        Ok(<$tp>::try_from(s.clone())?)
                    }
                    None => Err(anyhow!("absent value for key: {}", key)),
                }
            }
        }

        impl<'a, 'b> KDbRowBehavier<'b, Option<$tp>> for SqlValueRow<SqlValueOwned> {
            fn try_get(&'b self, key: &str) -> AResult<Option<$tp>> {
                match self.row.get(key) {
                    Some(value) => {
                        match value.deref() {
                            SqlValue::Opt(None) => {
                                Ok(None)
                            }
                            SqlValue::Opt(Some(v)) => {
                                Ok(Some(<$tp>::try_from(v.as_ref().clone())?))
                            }
                            v => {
                                Ok(Some(<$tp>::try_from(v.borrow().clone())?))
                            }
                        }
                    }
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
