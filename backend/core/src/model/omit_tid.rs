use std::ops::Deref;

use chin_sql::{time_type::TID, ChinSqlError, SqlUpdater, SqlValue};
use serde::{Deserialize, Serialize, Serializer};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OmitTID(pub(crate) TID);

impl OmitTID {
    #[inline]
    pub fn omitted(&self) -> bool {
        !self.0.is_never()
    }

    #[inline]
    pub fn never() -> Self {
        Self(TID::never())
    }

    #[inline]
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
        Option::<TID>::deserialize(deserializer).map(|e| match e {
            Some(e) => OmitTID(e),
            None => OmitTID::never(),
        })
    }
}

pub trait OmitNow {
    fn omit_now(self, key: &'static str) -> Self;
}

impl OmitNow for SqlUpdater<'_> {
    fn omit_now(self, key: &'static str) -> Self {
        self.set(key, OmitTID::now())
    }
}

impl From<OmitTID> for SqlValue<'_> {
    fn from(value: OmitTID) -> Self {
        Self::I64(value.0.as_num())
    }
}

impl<'a> TryFrom<SqlValue<'a>> for OmitTID {
    type Error = ChinSqlError;

    fn try_from(value: SqlValue<'a>) -> Result<Self, Self::Error> {
        let v: i64 = value.try_into()?;
        Ok(Self(v.into()))
    }
}

