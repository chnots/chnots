use std::str::FromStr;

use chin_sql::SqlValue;
use postgres_types::{to_sql_checked, FromSql, ToSql};

use crate::model::db::chnot::{ChnotKind, ChnotTagType};

impl<'a> From<ChnotTagType> for SqlValue<'a> {
    fn from(value: ChnotTagType) -> Self {
        SqlValue::I32(value as i32)
    }
}

#[macro_export]
macro_rules! to_sql {
    ($values:expr) => {
        $values
            .iter()
            .map(|e| {
                let v: &(dyn postgres_types::ToSql + Sync + Send) = e.into();
                v as &(dyn postgres_types::ToSql + Sync)
            })
            .collect::<Vec<&(dyn postgres_types::ToSql + Sync)>>()
            .as_slice()
    };
}

impl<'a> FromSql<'a> for ChnotKind {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw)
            .and_then(|s| Ok(ChnotKind::from_str(s)?))
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

impl ToSql for ChnotKind {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut tokio_util::bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
