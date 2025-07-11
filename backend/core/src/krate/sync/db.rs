use chin_sql::{SqlBuilder, Wheres};
use chin_tools::AResult;

use crate::{
    krate::sync::{dto::FetchDataType, mapper::Dumper},
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow},
};

impl Dumper<KDbRow> for KDb {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(KDbRow) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        let sql = SqlBuilder::read_all(table_name);
        let sql = match fetch_data {
            FetchDataType::RangePage {
                start_ex,
                end_in,
                page_size,
            } => {
                sql.r#where(Wheres::and([
                    // all table must have tid field
                    Wheres::compare("tid", ">", start_ex),
                    Wheres::compare("tid", "<=", end_in),
                ]))
                .limit(page_size)
            }
            FetchDataType::Tids(tids) => sql.r#where(Wheres::r#in("tid", tids)),
        };

        let rows = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(rows)
    }
}
