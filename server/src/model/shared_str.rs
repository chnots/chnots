use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SharedStr(SmolStr);

impl Deref for SharedStr {
    type Target = SmolStr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SharedStr {
    fn from(value: String) -> Self {
        Self(SmolStr::from(value))
    }
}

impl From<&'_ str> for SharedStr {
    fn from(value: &'_ str) -> Self {
        Self(SmolStr::from(value))
    }
}

impl AsRef<str> for SharedStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SharedStr {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(SmolStr::new(s))
    }
}

impl AsRef<[u8]> for SharedStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
