use std::{marker::PhantomData, ops::Deref};

use chin_sql::{SqlBuilder, SqlDeleter, Wheres, time_type::TID};
use chin_tools::AResult;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{
    mapper::{
        Curd,
        db::{
            KDb, KDbBehaiver, KDbConn, KDbExecutor, KDbExecutorBehaiver, KDbTransactionBehaiver,
            KDbTx,
        },
    },
    model::KSerde,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Sequence)]
pub enum OtidTableEnum {
    MdwtRecord,
    ChnotThreadOrder,
    MdwtTag,
    ChnotMeta,
    MdwtToent,
    LLMChatBot,
    LLMChatRecord,
    LLMChatTemplate,
    LLMChatSession,
    KSpace,
    #[allow(clippy::upper_case_acronyms)]
    KKV,
    KTabMeta,
    KTabCellDate,
    KTabCellDecimal,
    KTabCellText,
    KFileMeta, // inline k file is a specifal type file, so we sync it with kfilemeta
    GraphMeta, // graph data is a specifal type
}

#[macro_export]
macro_rules! otid_enum_to_generic {
    ($table_type:expr, $invoke:ident) => {{
        use $crate::model::otid_table::OtidTableEnum;
        match $table_type {
            OtidTableEnum::MdwtRecord => $invoke! {$crate::krate::mdwt::MdwtRecord},
            OtidTableEnum::MdwtTag => $invoke! {$crate::krate::mdwt::MdwtTag},
            OtidTableEnum::LLMChatBot => $invoke! {$crate::krate::llmchat::LLMChatBot},
            OtidTableEnum::LLMChatRecord => $invoke! {$crate::krate::llmchat::LLMChatRecord},
            OtidTableEnum::LLMChatTemplate => $invoke! {$crate::krate::llmchat::LLMChatTemplate},
            OtidTableEnum::LLMChatSession => $invoke! {$crate::krate::llmchat::LLMChatSession},
            OtidTableEnum::KKV => $invoke! {$crate::krate::kkv::KKV},
            OtidTableEnum::KTabMeta => $invoke! {$crate::krate::ktab::KTabMeta},
            OtidTableEnum::KTabCellDate => $invoke! {$crate::krate::ktab::KTabCellDate},
            OtidTableEnum::KTabCellDecimal => $invoke! {$crate::krate::ktab::KTabCellDecimal},
            OtidTableEnum::KTabCellText => $invoke! {$crate::krate::ktab::KTabCellText},
            OtidTableEnum::KFileMeta => $invoke! {$crate::krate::kfile::KFileMeta},
            OtidTableEnum::KSpace => $invoke! {$crate::krate::kspace::KSpace},
            OtidTableEnum::ChnotMeta => $invoke! {$crate::krate::chnot::ChnotMeta},
            OtidTableEnum::MdwtToent => $invoke! {$crate::krate::mdwt::MdwtRecord},
            OtidTableEnum::ChnotThreadOrder => $invoke! {$crate::krate::chnot::ChnotThreadOrder},
            OtidTableEnum::GraphMeta => $invoke! {$crate::krate::graph::GraphMeta},
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithEnum<E> {
    pub(crate) table_type: OtidTableEnum,
    pub(crate) dto: E,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithGeneric<E, T> {
    pub(crate) table_type: PhantomData<T>,
    pub(crate) dto: E,
}

impl<E, T> Deref for OtidWithGeneric<E, T> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.dto
    }
}

pub(crate) trait OtidTableSupport: KSerde + Curd {
    fn get_otid_enum() -> OtidTableEnum;
    fn table_name(hist: bool) -> &'static str;
    fn all_columns() -> &'static [&'static str];
}

#[macro_export]
macro_rules! impl_otid_support {
    ($st:tt) => {
        impl $st {
            pub const HIST_TABLE: &str = const_format::formatcp!("{}_hist", $st::TABLE);
        }

        impl $crate::model::KSerde for $st {
            fn sql_inserter(&'_ self) -> chin_sql::SqlInserter<'_> {
                self.clone().to_sql_inserter()
            }

            fn try_from_kdb_row(row: &$crate::mapper::db::KDbRow) -> chin_tools::AResult<Self> {
                Self::try_from(row)
            }
        }

        impl $crate::model::OtidTableSupport for $st {
            fn get_otid_enum() -> $crate::model::otid_table::OtidTableEnum {
                $crate::model::otid_table::OtidTableEnum::$st
            }
            fn table_name(hist: bool) -> &'static str {
                match hist {
                    true => Self::HIST_TABLE,
                    false => Self::TABLE,
                }
            }
            fn all_columns() -> &'static [&'static str] {
                Self::all_field_names()
            }
        }

        impl<'a> $crate::mapper::db::kdb::HistCreateSql<'a> for $st {
            fn hist_table() -> chin_sql::CreateTableSqlOwned {
                let mut create_table = Self::create_sql().to_owned_sql();
                let pkey = create_table.pkey.clone();
                create_table.pkey.clear();

                for ele in pkey {
                    create_table.keys.push((ele.clone(), vec![ele]));
                }

                let unikeys = create_table.unikeys.clone();
                create_table.unikeys.clear();
                for ele in unikeys {
                    if ele.1[0].to_lowercase() == "tid" && ele.1.len() == 1 {
                        create_table.unikeys.push(ele)
                    } else {
                        create_table.keys.push(ele);
                    }
                }

                create_table.table_name = Self::HIST_TABLE.to_string();

                create_table
            }

            fn main_table() -> chin_sql::CreateTableSqlOwned {
                Self::create_sql().to_owned_sql()
            }
        }
    };
}

pub enum OtidSearchType {
    Current,
    Hist,
    Both,
}

impl KDbConn {
    #[inline]
    pub(crate) async fn omit_rows<T: OtidTableSupport>(
        &mut self,
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        let tx = self.transaction().await?;
        let c = tx.omit_rows::<T>(condition).await?;
        tx.cmt().await?;
        Ok(c)
    }
}

impl KDbTx<'_> {
    #[inline]
    pub(crate) async fn omit_rows<T: OtidTableSupport>(
        &self,
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        if condition.empty() {
            return Ok(0);
        }
        let count = self
            .as_executor()
            .copy_into_omit_table::<T>(condition.clone())
            .await?;

        if count > 0 {
            let delete_sql = SqlDeleter::new(T::table_name(false)).r#where(condition);
            self.exec(delete_sql).await
        } else {
            Ok(0)
        }
    }

    #[inline]
    pub async fn po_otid_insert<T, S>(&self, pos: S) -> AResult<usize>
    where
        T: OtidTableSupport,
        S: Into<Vec<T>>,
    {
        let vs = pos.into();
        let mut count = 0;
        for ele in vs {
            self.omit_rows::<T>(ele.pkey()).await?;
            count += self
                .exec(
                    ele.sql_inserter()
                        .on_conflict(chin_sql::OnConflict::Default),
                )
                .await?;
        }

        Ok(count)
    }
}

impl KDbExecutor<'_> {
    #[inline]
    pub(crate) async fn copy_into_omit_table<T: OtidTableSupport>(
        &self,
        condition: Wheres<'_>,
    ) -> AResult<usize> {
        if condition.empty() {
            return Ok(0);
        }
        let fields_comma = T::all_columns().join(",");
        let insert_sql = SqlBuilder::new()
            .seg(format!(
                "insert into {}({}) select {} from {}",
                T::table_name(true),
                &fields_comma,
                &fields_comma,
                T::table_name(false)
            ))
            .r#where(condition.clone());

        let count = self.exec(insert_sql).await?;
        Ok(count)
    }

    pub async fn po_otid_list<T, S>(&self, pos: S, search_type: OtidSearchType) -> AResult<Vec<T>>
    where
        T: OtidTableSupport,
        S: Into<Vec<TID>>,
    {
        let vs = pos.into();

        let results = match search_type {
            OtidSearchType::Both => {
                self.qry_list(
                    SqlBuilder::new()
                        .seg("select * from")
                        .seg(T::table_name(true))
                        .r#where(Wheres::r#in("otid", vs.clone()))
                        .seg("union")
                        .seg("select * from")
                        .seg(T::table_name(false))
                        .r#where(Wheres::r#in("otid", vs)),
                    |row| T::try_from_kdb_row(&row),
                )
                .await?
            }
            _ => {
                self.qry_list(
                    SqlBuilder::new()
                        .seg("select * from")
                        .seg(T::table_name(match search_type {
                            OtidSearchType::Current => false,
                            OtidSearchType::Hist => true,
                            OtidSearchType::Both => unreachable!(),
                        }))
                        .r#where(Wheres::r#in("otid", vs)),
                    |row| T::try_from_kdb_row(&row),
                )
                .await?
            }
        };

        Ok(results)
    }
}

impl KDb {
    pub async fn po_otid_insert<T, S>(&self, pos: S) -> AResult<usize>
    where
        T: OtidTableSupport,
        S: Into<Vec<T>>,
    {
        let pos = pos.into();
        let mut count = 0;
        let mut conn = self.conn().await?;
        if pos.len() == 1 {
            let tx: KDbTx<'_> = conn.transaction().await?;

            count = tx.po_otid_insert(pos).await?;
            tx.cmt().await?;
        } else if !pos.is_empty() {
            let tx: KDbTx<'_> = conn.transaction().await?;
            count = tx.po_otid_insert(pos).await?;
            tx.cmt().await?;
        }

        Ok(count)
    }

    pub async fn po_otid_list<T, S>(&self, otids: S, search_type: OtidSearchType) -> AResult<Vec<T>>
    where
        T: OtidTableSupport,
        S: Into<Vec<TID>>,
    {
        self.conn()
            .await?
            .as_executor()
            .po_otid_list(otids, search_type)
            .await
    }
}
