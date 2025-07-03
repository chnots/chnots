use std::convert::TryFrom;

use anyhow::anyhow;
use chin_sql::SqlValue;
use postgres_types::FromSql;
use serde::{
    de::{self},
    Deserialize, Serialize,
};

use crate::mapper::db::{KDbRow, KDbRowBehavier};

#[derive(Debug, PartialEq, Clone)]
pub struct Decimal(String);

impl TryFrom<String> for Decimal {
    type Error = anyhow::Error;

    fn try_from(original_string: String) -> Result<Self, Self::Error> {
        let trimmed_value = original_string.trim();

        let is_valid = if trimmed_value.is_empty() {
            false
        } else {
            match trimmed_value.to_lowercase().as_str() {
                "pi" | "e" => true,
                _ => trimmed_value.parse::<f64>().is_ok(),
            }
        };

        if is_valid {
            Ok(Decimal(original_string))
        } else {
            Err(anyhow!(format!(
                "'{}' is not a valid number, scientific notation, or supported constant.",
                original_string
            )))
        }
    }
}

impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Decimal::try_from(s) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(de::Error::custom(err.to_string())),
        }
    }
}

impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        Self(value.to_string())
    }
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

impl From<Decimal> for SqlValue<'_> {
    fn from(value: Decimal) -> Self {
        SqlValue::Str(value.0.into())
    }
}

impl<'a> From<&'a Decimal> for SqlValue<'a> {
    fn from(value: &'a Decimal) -> Self {
        SqlValue::Str(value.0.as_str().into())
    }
}

impl<'a> KDbRowBehavier<'a, Decimal> for KDbRow {
    fn try_get(&'a self, key: &str) -> chin_tools::AResult<Decimal> {
        let s: String = self.try_get(key)?;

        Ok(Decimal(s))
    }
}
