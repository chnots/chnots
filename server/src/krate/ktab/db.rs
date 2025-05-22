use anyhow::Ok;
use chin_sql::{SqlInserter, SqlReader, SqlUpdater, Wheres};
use chrono::Local;

use crate::{
    mapper::db::{KDb, KDbBehaiver, KDbConnBehaiver, KDbRowBehavier},
    model::dto::KReq,
};

use super::{mapper::ChinTableMapper, *};

impl ChinTableMapper for KDb {
    async fn ctable_overwrite_meta(
        &self,
        req: KReq<CTableOverwriteMetaReq>,
    ) -> chin_tools::AResult<CTableOverwriteMetaRsp> {
        let CTableMeta {
            id,
            columns,
            table_name,
            table_comment,
            create_time,
            update_time: _,
            delete_time: _,
            real_table,
            workspace: _,
        } = &req.meta;

        let omit_sql = SqlUpdater::new(CTableMeta::TABLE)
            .set(CTableMeta::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::and([Wheres::equal(
                CTableMeta::TABLE_NAME,
                table_name,
            )]));

        let insert_sql = SqlInserter::new(CTableMeta::TABLE)
            .fields(CTableMeta::ID, id)
            .fields(CTableMeta::COLUMNS, serde_json::to_string(&columns)?)
            .fields(CTableMeta::TABLE_NAME, table_name)
            .fields(CTableMeta::CREATE_TIME, create_time)
            .fields(CTableMeta::TABLE_COMMENT, table_comment)
            .fields(CTableMeta::REAL_TABLE, *real_table);

        self.conn().await?.exec(omit_sql).await?;
        self.conn().await?.exec(insert_sql).await?;

        Ok(CTableOverwriteMetaRsp {})
    }

    async fn ctable_overwrite_row(
        &self,
        req: KReq<CTableOverwriteRowReq>,
    ) -> chin_tools::AResult<CTableOverwriteRowRsp> {
        let emtpy_req = req.frame(());
        let KReq { body, workspace: _ } = req;

        let CTableOverwriteRowReq { row } = body;
        for ele in row {
            self.ctable_overwrite_cell(emtpy_req.frame(CTableOverwriteCellReq { cell: ele }))
                .await?;
        }

        Ok(CTableOverwriteRowRsp {})
    }

    async fn ctable_overwrite_cell(
        &self,
        req: KReq<CTableOverwriteCellReq>,
    ) -> chin_tools::AResult<CTableOverwriteCellRsp> {
        macro_rules! overwrite {
            ($table:tt, $c:expr) => {
                let csql = SqlInserter::new($table::TABLE)
                    .fields($table::TABLE_ID, &$c.table_id)
                    .fields($table::COL_IDX, $c.col_idx)
                    .fields($table::ROW_IDX, $c.row_idx)
                    .fields($table::INSERT_TIME, &$c.insert_time);
                let omit_sql = SqlUpdater::new($table::TABLE)
                    .set($table::DELETE_TIME, Local::now().fixed_offset())
                    .r#where(Wheres::and([
                        Wheres::equal($table::TABLE_ID, &$c.table_id),
                        Wheres::equal($table::ROW_IDX, $c.row_idx),
                        Wheres::equal($table::COL_IDX, $c.col_idx),
                    ]));
                self.conn().await?.exec(omit_sql).await?;
                self.conn().await?.exec(csql).await?;
            };
        }

        match &req.cell {
            CTableCell::String(c) => {
                overwrite!(CTableCellStr1024, c);
            }
            CTableCell::Text(c) => {
                overwrite!(CTableCellText, c);
            }
            CTableCell::Integer(c) => {
                overwrite!(CTableCellInteger, c);
            }
            CTableCell::Date(c) => {
                overwrite!(CTableCellDate, c);
            }
        }

        Ok(CTableOverwriteCellRsp {})
    }

    async fn ctable_query_row(
        &self,
        req: KReq<CTableQueryRowReq>,
    ) -> chin_tools::AResult<CTableQueryRowRsp> {
        let mut cells = vec![];
        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader = SqlReader::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::ROW_IDX, req.row_index),
                    Wheres::equal($sub_table::TABLE_ID, req.table_id.as_str()),
                    Wheres::is_null($sub_table::DELETE_TIME),
                ]));

                let data: Vec<CTableCell> = self
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

        extend_cells!(CTableCellStr1024);
        extend_cells!(CTableCellDate);
        extend_cells!(CTableCellInteger);
        extend_cells!(CTableCellText);

        Ok(CTableQueryRowRsp { row: cells })
    }

    async fn ctable_query_table_meta(
        &self,
        req: KReq<CTableQueryTableMetaReq>,
    ) -> chin_tools::AResult<CTableQueryTableMetaRsp> {
        let ssb = SqlReader::read_all(CTableMeta::TABLE).r#where(Wheres::and([
            Wheres::equal(CTableMeta::TABLE, &req.table_name),
            Wheres::is_null(CTableMeta::DELETE_TIME),
        ]));
        let meta = self
            .conn()
            .await?
            .qry_one(
                ssb,
                |row| {
                    Ok(CTableMeta {
                        columns: {
                            let columns: String = row.try_get(CTableMeta::COLUMNS)?;
                            serde_json::from_str(&columns)?
                        },
                        table_name: row.try_get(CTableMeta::TABLE_NAME)?,
                        table_comment: row.try_get(CTableMeta::TABLE_COMMENT)?,
                        create_time: row.try_get(CTableMeta::CREATE_TIME)?,
                        update_time: row.try_get(CTableMeta::UPDATE_TIME)?,
                        delete_time: row.try_get(CTableMeta::DELETE_TIME)?,
                        real_table: row.try_get(CTableMeta::REAL_TABLE)?,
                        id: row.try_get(CTableMeta::ID)?,
                        workspace: row.try_get(CTableMeta::WORKSPACE)?,
                    })
                },
                true,
            )
            .await?;

        Ok(CTableQueryTableMetaRsp { meta })
    }

    async fn ctable_query_table_data(
        &self,
        req: KReq<CTableQueryTableDataReq>,
    ) -> chin_tools::AResult<CTableQueryTableDataRsp> {
        let mut cells = vec![];
        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader = SqlReader::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::TABLE_ID, req.table_id.as_str()),
                    Wheres::is_null($sub_table::DELETE_TIME),
                ]));

                let data: Vec<CTableCell> = self
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

        extend_cells!(CTableCellStr1024);
        extend_cells!(CTableCellDate);
        extend_cells!(CTableCellInteger);
        extend_cells!(CTableCellText);

        Ok(CTableQueryTableDataRsp { cells })
    }

    async fn ensure_ctable_tables(&self) -> chin_tools::EResult {
        self.conn()
            .await?
            .exec(CTableCellDate::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(CTableCellInteger::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(CTableCellStr1024::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(CTableCellText::schema(self.db_type()))
            .await?;
        self.conn()
            .await?
            .exec(CTableMeta::schema(self.db_type()))
            .await?;

        Ok(())
    }
}
