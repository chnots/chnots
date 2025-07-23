use std::collections::HashMap;

use anyhow::{Context, Ok};
use chin_sql::{SqlBuilder, SqlInserter, Wheres, time_type::TID};
use chin_tools::AResult;
use itertools::Itertools;
use log::info;

use crate::{
    mapper::db::{
        helper::create_tables, HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTransactionBehaiver
    },
    model::dto::KReq,
};

use super::{mapper::KTabMapper, *};

impl KDb {
    async fn ktab_overwrite_cell(&self, cell: KTabCell) -> chin_tools::AResult<usize> {
        macro_rules! overwrite {
            ($table:tt, $c:expr, $v:expr) => {
                let csql = SqlInserter::new($table::TABLE)
                    .field($table::TABLE_OTID, $c.table_otid)
                    .field($table::COL_OTID, $c.col_otid)
                    .field($table::ROW_OTID, $c.row_otid)
                    .field($table::CELL_DATA, $v)
                    .field($table::TID, $c.tid);
                let mut conn = self.conn().await?;
                let tx = conn.tx().await?;                
                tx.as_executor()
                    .omit_rows(
                        $table::TABLE,
                        &$table::create_sql().all_fields(),
                        $table::pkey_cond($c.table_otid, $c.col_otid, $c.row_otid),
                    )
                    .await?;
                tx.exec(csql).await?;
                tx.cmt().await?;
            };
        }

        match &cell.cell_data {
            KTabStoreValue::Text(c) => {
                overwrite!(KTabCellText, cell, c);
            }
            KTabStoreValue::Decimal(c) => {
                overwrite!(KTabCellDecimal, cell, c);
            }
            KTabStoreValue::Date(c) => {
                overwrite!(KTabCellDate, cell, c);
            }
        }

        Ok(1)
    }
}

impl KTabMapper for KDb {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabMetaOverwriteReq>,
    ) -> chin_tools::AResult<KTabMetaOverwriteRsp> {
        let KTabMeta {
            otid,
            columns,
            table_name,
            table_comment,
            update_time: _,
            real_table,
            tid: _,
        } = &req.meta;
        let full = columns.values().map(|e| e.idx).collect_vec();
        if full.iter().unique().count() < full.len() {
            anyhow::bail!("the column indexes are not unique.");
        }

        let insert_sql = SqlInserter::new(KTabMeta::TABLE)
            .field(KTabMeta::OTID, *otid)
            .field(KTabMeta::TID, TID::default())
            .field(KTabMeta::COLUMNS, serde_json::to_string(&columns)?)
            .field(KTabMeta::TABLE_NAME, table_name.clone())
            .field(KTabMeta::TABLE_COMMENT, table_comment.clone())
            .field(KTabMeta::REAL_TABLE, *real_table)
            .on_conflict(chin_sql::OnConflict::Replace([KTabMeta::OTID].join(", ")));
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows(
                KTabMeta::TABLE,
                &KTabMeta::create_sql().all_fields(),
                KTabMeta::pkey_cond(*otid),
            )
            .await?;
        tx.exec(insert_sql).await?;
        tx.cmt().await?;

        Ok(KTabMetaOverwriteRsp {})
    }

    async fn ktab_overwrite_cells(
        &self,
        req: KReq<KTabCellsOverwriteReq>,
    ) -> chin_tools::AResult<KTabCellsOverwriteRsp> {
        info!("req -- {:?}", req);
        let empty_wrapper = req.frame(());
        let KReq {
            body,
            kspace: _,
            mkspaces: _,
        } = req;

        let KTabCellsOverwriteReq {
            cells,
            table_id,
        } = body;
        let table_meta = self
            .ktab_query_table_meta(empty_wrapper.frame(KTabMetaQueryReq { table_id }))
            .await?
            .meta
            .context("unable to get this table")?;
        let table_otid = table_meta.otid;

        let columns = table_meta.columns;

        for ele in cells {
            let column_index = columns
                .get(ele.column_name.as_str())
                .context("the column is not existed")?
                .idx;

            self.ktab_overwrite_cell(KTabCell {
                table_otid,
                col_otid: column_index,
                row_otid: ele.row_tid,
                cell_data: ele.value,
                tid: TID::default(),
            })
            .await?;
        }

        Ok(KTabCellsOverwriteRsp {})
    }

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabMetaQueryReq>,
    ) -> chin_tools::AResult<KTabMetaQueryRsp> {
        let ssb = SqlBuilder::read_all(KTabMeta::TABLE)
            .r#where(Wheres::and([Wheres::equal(KTabMeta::OTID, req.table_id)]));
        let meta = self
            .conn()
            .await?
            .qry_opt(ssb, |row| row.try_into())
            .await?;

        Ok(KTabMetaQueryRsp { meta })
    }

    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabRowsQueryReq>,
    ) -> chin_tools::AResult<KTabRowsQueryRsp> {
        let mut cells = vec![];
        let table_id = req.table_id;
        let config = self
            .ktab_query_table_meta(req.frame(KTabMetaQueryReq {
                table_id: req.table_id,
            }))
            .await?
            .meta
            .context("cannot find table")?;
        let col_names: HashMap<TID, String> = config
            .columns
            .into_values()
            .map(|c| (c.idx, c.name))
            .collect();

        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader =
                    SqlBuilder::read_all($sub_table::TABLE).r#where(Wheres::and([Wheres::equal(
                        $sub_table::TABLE_OTID,
                        table_id,
                    )]));

                let data: Vec<KTabCell> = self
                    .conn()
                    .await?
                    .qry_list(reader, |row| {
                        let t: $sub_table = row.try_into()?;
                        Ok(t)
                    })
                    .await?
                    .into_iter()
                    .map(|cell| cell.into())
                    .collect();
                cells.extend(data);
            };
        }

        extend_cells!(KTabCellText);
        extend_cells!(KTabCellDate);
        extend_cells!(KTabCellDecimal);

        let cells: AResult<Vec<KTabViewCell>> = cells
            .into_iter()
            .map(|cell| {
                cell.into_view(&col_names)
                    .context("unable find this column")
            })
            .collect();
        let cells = cells?;
        let mut result_map = HashMap::new();
        for cell in cells {
            result_map.entry(cell.row_tid).or_insert(vec![]).push(cell);
        }
        let result = result_map
            .into_iter()
            .map(|(k, r)| KTabRowsQueryRspRow {
                row_tid: k,
                cells: r,
            })
            .sorted_by(|r1, r2| r1.row_tid.cmp(&r2.row_tid))
            .collect();

        Ok(KTabRowsQueryRsp { rows: result })
    }

    async fn ensure_ktab_tables(&self) -> chin_tools::EResult {
        create_tables(
            vec![
                KTabMeta::create_sql().to_owned_sql(),
                KTabMeta::hist_table(),
                KTabCellText::create_sql().to_owned_sql(),
                KTabCellText::hist_table(),
                KTabCellDecimal::create_sql().to_owned_sql(),
                KTabCellDecimal::hist_table(),
                KTabCellDate::create_sql().to_owned_sql(),
                KTabCellDate::hist_table(),
            ],
            self,
        )
        .await
    }
}

impl TryFrom<KDbRow> for KTabMeta {
    type Error = anyhow::Error;

    fn try_from(row: KDbRow) -> Result<Self, Self::Error> {
        Ok(KTabMeta {
            columns: {
                let columns: String = row.try_get(KTabMeta::COLUMNS)?;
                serde_json::from_str(&columns)?
            },
            table_name: row.try_get(KTabMeta::TABLE_NAME)?,
            table_comment: row.try_get(KTabMeta::TABLE_COMMENT)?,
            update_time: row.try_get(KTabMeta::UPDATE_TIME)?,
            real_table: row.try_get(KTabMeta::REAL_TABLE)?,
            otid: row.try_get(KTabMeta::OTID)?,
            tid: row.try_get(KTabMeta::TID)?,
        })
    }
}

macro_rules! row_into_ktab_cell {
    ($sub_table:tt) => {
        impl TryFrom<KDbRow> for $sub_table {
            type Error = anyhow::Error;

            fn try_from(row: KDbRow) -> Result<Self, Self::Error> {
                Ok($sub_table {
                    table_otid: row.try_get($sub_table::TABLE_OTID)?,
                    col_otid: row.try_get($sub_table::COL_OTID)?,
                    row_otid: row.try_get($sub_table::ROW_OTID)?,
                    cell_data: row.try_get($sub_table::CELL_DATA)?,
                    tid: row.try_get($sub_table::TID)?,
                })
            }
        }
    };
}

row_into_ktab_cell!(KTabCellDate);
row_into_ktab_cell!(KTabCellDecimal);
row_into_ktab_cell!(KTabCellText);
