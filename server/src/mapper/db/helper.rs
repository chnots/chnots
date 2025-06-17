use chin_sql::SqlValue;
use chin_tools::AResult;

pub(crate) trait HighLevelText<'a> {
    fn try_from_sql(s: &str) -> AResult<Self>
    where
        Self: std::marker::Sized;
    fn into_sql(self) -> SqlValue<'a>;
}
