use chin_sql::{SqlBuilder, Wheres, str_type::Varchar};
use chin_tools::AResult;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{
    mapper::{
        Curd,
        db::{KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver, KDbTransactionBehaiver},
    },
    model::KSerde,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Sequence)]
pub enum SidTableEnum {
    InlineKFile,
    GraphData,
}

#[macro_export]
macro_rules! sid_enum_to_generic {
    ($table_type:expr, $invoke:ident) => {{
        use $crate::model::SidTableEnum;
        match $table_type {
            SidTableEnum::InlineKFile => $invoke! {$crate::krate::kfile::InlineKFile},
            SidTableEnum::GraphData => $invoke! {$crate::krate::graph::GraphData},
        }
    }};
}

pub(crate) trait SidTableSupport: KSerde + Curd {
    fn get_sid_enum() -> SidTableEnum;

    fn table_name() -> &'static str;
    fn all_columns() -> &'static [&'static str];
}

#[macro_export]
macro_rules! impl_sid_support {
    ($st:tt) => {
        impl $crate::mapper::Curd for $st {
            fn pkey(&self) -> chin_sql::Wheres<'_> {
                Self::pkey_cond(self.sid.clone())
            }

            fn tid(&self) -> TID {
                self.tid
            }
        }

        impl $crate::model::KSerde for $st {
            fn sql_inserter(&'_ self) -> chin_sql::SqlInserter<'_> {
                self.clone().to_sql_inserter()
            }

            fn try_from_kdb_row(row: &$crate::mapper::db::KDbRow) -> chin_tools::AResult<Self> {
                Self::try_from(row)
            }
        }

        impl $crate::model::SidTableSupport for $st {
            fn get_sid_enum() -> $crate::model::SidTableEnum {
                $crate::model::SidTableEnum::$st
            }

            #[inline]
            fn table_name() -> &'static str {
                Self::TABLE
            }
            #[inline]
            fn all_columns() -> &'static [&'static str] {
                Self::all_field_names()
            }
        }
    };
}

impl KDbExecutor<'_> {
    pub async fn po_sid_insert<T, S>(&self, pos: S) -> AResult<usize>
    where
        T: SidTableSupport,
        S: Into<Vec<T>>,
    {
        let vs = pos.into();
        let mut count = 0;
        for ele in vs {
            count += self
                .exec(ele.sql_inserter().on_conflict(chin_sql::OnConflict::Ignore))
                .await?;
        }

        Ok(count)
    }

    pub async fn po_sid_list<const LIMIT: usize, T, S>(&self, pos: S) -> AResult<Vec<T>>
    where
        T: SidTableSupport,
        S: Into<Vec<Varchar<LIMIT>>>,
    {
        let vs = pos.into();
        if vs.is_empty() {
            return Ok(vec![]);
        }

        let results = self
            .qry_list(
                SqlBuilder::new()
                    .seg("select * from")
                    .seg(T::table_name())
                    .r#where(Wheres::r#in("sid", vs)),
                |row| T::try_from_kdb_row(&row),
            )
            .await?;

        Ok(results)
    }
}

impl KDb {
    pub async fn po_sid_insert<T, S>(&self, pos: S) -> AResult<usize>
    where
        T: SidTableSupport,
        S: Into<Vec<T>>,
    {
        let pos = pos.into();
        let mut count = 0;
        if pos.len() == 1 {
            count = self.conn().await?.as_executor().po_sid_insert(pos).await?;
        } else if !pos.is_empty() {
            let mut conn = self.conn().await?;
            let tx = conn.transaction().await?;
            count = tx.as_executor().po_sid_insert(pos).await?;
            tx.cmt().await?;
        }
        Ok(count)
    }

    pub async fn po_sid_list<const LIMIT: usize, T, S>(&self, sids: S) -> AResult<Vec<T>>
    where
        T: SidTableSupport,
        S: Into<Vec<Varchar<LIMIT>>>,
    {
        self.conn().await?.as_executor().po_sid_list(sids).await
    }
}
