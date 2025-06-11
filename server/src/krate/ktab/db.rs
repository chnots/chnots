use std::collections::HashMap;

use anyhow::{Context, Ok};
use chin_sql::{SqlInserter, SqlReader, SqlUpdater, Wheres};
use chin_tools::AResult;
use chrono::Local;
use itertools::Itertools;

use crate::{
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier},
    model::dto::KReq,
};

use super::{mapper::KTabMapper, *};

impl KDb {
    async fn ktab_overwrite_cell(&self, cell: KTabCell) -> chin_tools::AResult<usize> {
        macro_rules! overwrite {
            ($table:tt, $c:expr, $v:expr) => {
                let csql = SqlInserter::new($table::TABLE)
                    .field($table::TABLE_ID, $c.table_id)
                    .field($table::COL_IDX, $c.col_idx)
                    .field($table::ROW_IDX, $c.row_idx)
                    .field($table::CELL_DATA, $v)
                    .field($table::INSERT_TIME, &$c.insert_time);
                let omit_sql = SqlUpdater::new($table::TABLE)
                    .set($table::DELETE_TIME, Local::now().fixed_offset())
                    .r#where(Wheres::and([
                        Wheres::equal($table::TABLE_ID, $c.table_id),
                        Wheres::equal($table::ROW_IDX, $c.row_idx),
                        Wheres::equal($table::COL_IDX, $c.col_idx),
                    ]));
                self.conn().await?.exec(omit_sql).await?;
                self.conn().await?.exec(csql).await?;
            };
        }

        match &cell.cell_data {
            KTabStoreValue::Text(c) => {
                overwrite!(KTabCellText, cell, c);
            }
            KTabStoreValue::I64(c) => {
                overwrite!(KTabCellI64, cell, *c);
            }
            KTabStoreValue::Date(c) => {
                overwrite!(KTabCellDate, cell, c);
            }
            KTabStoreValue::F64(c) => {
                overwrite!(KTabCellF64, cell, *c);
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
            id,
            columns,
            table_name,
            table_comment,
            create_time,
            update_time: _,
            delete_time: _,
            real_table,
            kspace,
        } = &req.meta;
        let full = columns.values().map(|e| e.idx).collect_vec();
        if full.iter().unique().count() < full.len() {
            anyhow::bail!("the column indexes are not unique.");
        }

        let omit_sql = SqlUpdater::new(KTabMeta::TABLE)
            .set(KTabMeta::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::and([Wheres::equal(
                KTabMeta::TABLE_NAME,
                table_name,
            )]));

        let insert_sql = SqlInserter::new(KTabMeta::TABLE)
            .field(KTabMeta::ID, *id)
            .field(KTabMeta::COLUMNS, serde_json::to_string(&columns)?)
            .field(KTabMeta::TABLE_NAME, table_name)
            .field(KTabMeta::CREATE_TIME, create_time)
            .field(KTabMeta::TABLE_COMMENT, table_comment)
            .field(KTabMeta::KSPACE, kspace)
            .field(KTabMeta::REAL_TABLE, *real_table)
            .on_conflict(chin_sql::OnConflict::Replace(KTabMeta::ID.to_owned()));

        self.conn().await?.exec(omit_sql).await?;
        self.conn().await?.exec(insert_sql).await?;

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
        let table_id = table_meta.id;

        let columns = table_meta.columns;

        for ele in row {
            let column_index = columns
                .get(ele.column_name.as_str())
                .context("the column is not existed")?
                .idx;

            self.ktab_overwrite_cell(KTabCell {
                table_id: table_id.clone(),
                col_idx: column_index,
                row_idx: ele.row_idx,
                insert_time: Local::now().fixed_offset(),
                delete_time: None,
                cell_data: ele.value,
            })
            .await?;
        }

        Ok(KTabCellsOverwriteRsp {})
    }

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabMetaQueryReq>,
    ) -> chin_tools::AResult<KTabMetaQueryRsp> {
        let ssb = SqlReader::read_all(KTabMeta::TABLE).r#where(Wheres::and([
            Wheres::equal(KTabMeta::ID, req.table_id),
            Wheres::is_null(KTabMeta::DELETE_TIME),
            Wheres::equal(KTabMeta::KSPACE, &req.kspace),
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
                    create_time: row.try_get(KTabMeta::CREATE_TIME)?,
                    update_time: row.try_get(KTabMeta::UPDATE_TIME)?,
                    delete_time: row.try_get(KTabMeta::DELETE_TIME)?,
                    real_table: row.try_get(KTabMeta::REAL_TABLE)?,
                    id: row.try_get(KTabMeta::ID)?,
                    kspace: row.try_get(KTabMeta::KSPACE)?,
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
        let col_names: HashMap<i64, String> = config
            .columns
            .into_values()
            .map(|c| (c.idx, c.name))
            .collect();

        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader = SqlReader::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::TABLE_ID, table_id),
                    Wheres::is_null($sub_table::DELETE_TIME),
                ]));

                let data: Vec<KTabCell> = self
                    .conn()
                    .await?
                    .qry_list(reader, |row| {
                        Ok($sub_table {
                            table_id: row.try_get($sub_table::TABLE_ID)?,
                            col_idx: row.try_get($sub_table::COL_IDX)?,
                            row_idx: row.try_get($sub_table::ROW_IDX)?,
                            cell_data: row.try_get($sub_table::CELL_DATA)?,
                            insert_time: row.try_get($sub_table::INSERT_TIME)?,
                            delete_time: row.try_get($sub_table::DELETE_TIME)?,
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
        extend_cells!(KTabCellI64);
        extend_cells!(KTabCellF64);

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
            result_map.entry(cell.row_idx).or_insert(vec![]).push(cell);
        }
        let result = result_map
            .into_iter()
            .map(|(k, r)| KTabRowsQueryRspRow {
                row_idx: k,
                cells: r,
            })
            .sorted_by(|r1, r2| r1.row_idx.cmp(&r2.row_idx))
            .collect();

        Ok(KTabRowsQueryRsp { rows: result })
    }

    async fn ensure_ktab_tables(&self) -> chin_tools::EResult {
        self.conn()
            .await?
            .exec(KTabCellDate::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(KTabCellI64::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(KTabCellF64::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(KTabCellText::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(KTabMeta::schema(self.db_type()))
            .await?;

        Ok(())
    }
}
