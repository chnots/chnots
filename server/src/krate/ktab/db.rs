use std::collections::HashMap;

use anyhow::{Context, Ok};
use chin_sql::{SqlBuilder, SqlInserter, Wheres, time_type::TID};
use chin_tools::AResult;
use itertools::Itertools;

use crate::{
    mapper::db::{
        helper::create_tables, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRowBehavier, KDbTransactionBehaiver
    },
    model::{dto::KReq, omit_tid::OmitTID},
};

use super::{mapper::KTabMapper, *};

impl KDb {
    async fn ktab_overwrite_cell(&self, cell: KTabCell) -> chin_tools::AResult<usize> {
        macro_rules! overwrite {
            ($table:tt, $c:expr, $v:expr) => {
                let mut conn = self.conn().await?;
                let tx = conn.tx().await?;
                let csql = SqlInserter::new($table::TABLE)
                    .field($table::TABLE_ID, $c.table_id)
                    .field($table::COL_TID, $c.col_tid)
                    .field($table::ROW_TID, $c.row_tid)
                    .field($table::OMIT_TID, $c.omit_tid)
                    .field($table::CELL_DATA, $v)
                    .field($table::TID, $c.tid);
                let omit_sql =
                    $table::pkey_updater($c.table_id, $c.row_tid, $c.col_tid, OmitTID::never())
                        .set($table::OMIT_TID, OmitTID::now());
                tx.exec(omit_sql).await?;
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
            tid,
            columns,
            table_name,
            table_comment,
            update_time: _,
            omit_tid,
            real_table,
        } = &req.meta;
        let full = columns.values().map(|e| e.idx).collect_vec();
        if full.iter().unique().count() < full.len() {
            anyhow::bail!("the column indexes are not unique.");
        }

        let omit_sql =
            KTabMeta::pkey_updater(*tid, OmitTID::never()).set(KTabMeta::OMIT_TID, OmitTID::now());

        let insert_sql = SqlInserter::new(KTabMeta::TABLE)
            .field(KTabMeta::TID, *tid)
            .field(KTabMeta::COLUMNS, serde_json::to_string(&columns)?)
            .field(KTabMeta::TABLE_NAME, table_name.clone())
            .field(KTabMeta::TABLE_COMMENT, table_comment.clone())
            .field(KTabMeta::REAL_TABLE, *real_table)
            .field(KTabMeta::OMIT_TID, *omit_tid)
            .on_conflict(chin_sql::OnConflict::Replace(
                [KTabMeta::TID, KTabMeta::OMIT_TID].join(", "),
            ));
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.exec(omit_sql).await?;
        tx.exec(insert_sql).await?;
        tx.cmt().await?;

        Ok(KTabMetaOverwriteRsp {})
    }

    async fn ktab_overwrite_cells(
        &self,
        req: KReq<KTabCellsOverwriteReq>,
    ) -> chin_tools::AResult<KTabCellsOverwriteRsp> {
        let empty_wrapper = req.frame(());
        let KReq {
            body,
            kspace: _,
            mkspaces: _,
        } = req;

        let KTabCellsOverwriteReq {
            cells: row,
            table_id,
        } = body;
        let table_meta = self
            .ktab_query_table_meta(empty_wrapper.frame(KTabMetaQueryReq { table_id }))
            .await?
            .meta
            .context("unable to get this table")?;
        let table_id = table_meta.tid;

        let columns = table_meta.columns;

        for ele in row {
            let column_index = columns
                .get(ele.column_name.as_str())
                .context("the column is not existed")?
                .idx;

            self.ktab_overwrite_cell(KTabCell {
                table_id,
                col_tid: column_index,
                row_tid: ele.row_tid,
                omit_tid: OmitTID::never(),
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
        let ssb = SqlBuilder::read_all(KTabMeta::TABLE).r#where(Wheres::and([
            Wheres::equal(KTabMeta::TID, req.table_id),
            Wheres::equal(KTabMeta::OMIT_TID, OmitTID::never()),
        ]));
        let meta = self
            .conn()
            .await?
            .qry_opt(ssb, |row| {
                Ok(KTabMeta {
                    columns: {
                        let columns: String = row.try_get(KTabMeta::COLUMNS)?;
                        serde_json::from_str(&columns)?
                    },
                    table_name: row.try_get(KTabMeta::TABLE_NAME)?,
                    table_comment: row.try_get(KTabMeta::TABLE_COMMENT)?,
                    update_time: row.try_get(KTabMeta::UPDATE_TIME)?,
                    omit_tid: row.try_get(KTabMeta::OMIT_TID)?,
                    real_table: row.try_get(KTabMeta::REAL_TABLE)?,
                    tid: row.try_get(KTabMeta::TID)?,
                })
            })
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
                let reader = SqlBuilder::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::TABLE_ID, table_id),
                    Wheres::equal($sub_table::OMIT_TID, OmitTID::never()),
                ]));

                let data: Vec<KTabCell> = self
                    .conn()
                    .await?
                    .qry_list(reader, |row| {
                        Ok($sub_table {
                            table_id: row.try_get($sub_table::TABLE_ID)?,
                            col_tid: row.try_get($sub_table::COL_TID)?,
                            row_tid: row.try_get($sub_table::ROW_TID)?,
                            cell_data: row.try_get($sub_table::CELL_DATA)?,
                            tid: row.try_get($sub_table::TID)?,
                            omit_tid: row.try_get($sub_table::OMIT_TID)?,
                        })
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
                KTabMeta::create_sql(),
                KTabCellText::create_sql(),
                KTabCellDecimal::create_sql(),
                KTabCellDate::create_sql(),
            ],
            self,
        )
        .await
    }
}
