use chin_sql::{SqlBuilder, Wheres};
use chin_tools::AResult;

use crate::{
    krate::sync::mapper::Dumper,
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow},
};

impl Dumper<KDbRow> for KDb {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        start_tid: chin_sql::time_type::TID,
        page_size: usize,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(KDbRow) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        let rows = self
            .conn()
            .await?
            .qry_list(
                SqlBuilder::read_all(table_name)
                    .r#where(Wheres::and([
                        // all table must have tid field
                        Wheres::compare("tid", ">", start_tid),
                    ]))
                    .limit(page_size),
                mapper,
            )
            .await?;

        Ok(rows)
    }
}
