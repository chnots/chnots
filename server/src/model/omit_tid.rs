use std::ops::Deref;

use chin_sql::{time_type::TID, SqlValue, SqlValueOwned, SqlValueRow};
use serde::{Deserialize, Serialize, Serializer};

use crate::mapper::db::KDbRowBehavier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OmitTID(TID);

impl OmitTID {
    pub fn omitted(&self) -> bool {
        self.0.is_very_very_big()
    }

    pub fn never() -> Self {
        Self(TID::very_very_big())
    }

    pub fn now() -> Self {
        Self(TID::default())
    }
}

impl Deref for OmitTID {
    type Target = TID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for OmitTID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.omitted() {
            serializer.serialize_none()
        } else {
            serializer.serialize_i64(self.0.as_num())
        }
    }
}

impl<'a> Deserialize<'a> for OmitTID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Option::<TID>::deserialize(deserializer).map(|e| {
            match e {
                Some(e) => OmitTID(e),
                None => OmitTID::never(),
            }
        })
    }
}

impl From<OmitTID> for SqlValue<'_> {
    fn from(value: OmitTID) -> Self {
        Self::I64(value.0.as_num())
    }
}

pub mod pg {
    use chin_sql::time_type::TID;
    use postgres_types::{accepts, FromSql, Type};

    use crate::model::omit_tid::OmitTID;

    impl<'a> FromSql<'a> for OmitTID {
        fn from_sql(
            ty: &Type,
            raw: &'a [u8],
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            TID::from_sql(ty, raw).map(OmitTID)
        }

        accepts! {INT2, INT4, INT8}
    }
}

impl KDbRowBehavier<OmitTID> for SqlValueRow<SqlValueOwned> {
    fn try_get(&self, key: &str) -> chin_tools::AResult<OmitTID> {
        let tid: i64 = self.try_get(key)?;
        Ok(OmitTID(TID::from(tid)))
    }
}
