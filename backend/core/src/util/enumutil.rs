#[macro_export]
macro_rules! enum_common_funcs {
    ($st:ty) => {
        impl AsRef<str> for $st {
            fn as_ref(&self) -> &str {
                self.as_static_str()
            }
        }

        impl TryFrom<&str> for $st {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                use chin_tools::AnyhowContext;
                enum_iterator::all::<Self>()
                    .find(|e| e.as_ref().eq_ignore_ascii_case(value))
                    .context(format!("invalid string `{value}`"))
            }
        }

        impl serde::Serialize for $st {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.as_ref().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $st {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Self::try_from(s.as_str()).map_err(|err| serde::de::Error::custom(err.to_string()))
            }
        }
    };
}
