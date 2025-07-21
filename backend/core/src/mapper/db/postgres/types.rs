use chin_sql::time_type::TID;
use postgres_types::{FromSql, Type, accepts};

use crate::model::{decimal::Decimal};

#[macro_export]
macro_rules! to_pgsql_params {
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

impl<'a> FromSql<'a> for Decimal {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s: String = String::from_sql(ty, raw)?;
        Ok(Decimal::try_from(s)?)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        String::accepts(ty)
    }
}