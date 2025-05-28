use anyhow::Ok;
use chin_sql::{SqlInserter, SqlReader, SqlUpdater, Wheres};
use chrono::Local;

use crate::{
    mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbConnBehaiver, KDbRowBehavier, KDbTx},
    model::dto::KReq,
};

use super::{mapper::KTabMapper, *};

impl KTabMapper for KDb {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabOverwriteMetaReq>,
    ) -> chin_tools::AResult<KTabOverwriteMetaRsp> {
        let KTabMeta {
            id,
            columns,
            table_name,
            table_comment,
            create_time,
            update_time: _,
            delete_time: _,
            real_table,
            kspace: _,
        } = &req.meta;

        let omit_sql = SqlUpdater::new(KTabMeta::TABLE)
            .set(KTabMeta::DELETE_TIME, Local::now().fixed_offset())
            .r#where(Wheres::and([Wheres::equal(
                KTabMeta::TABLE_NAME,
                table_name,
            )]));

        let insert_sql = SqlInserter::new(KTabMeta::TABLE)
            .field(KTabMeta::ID, id)
            .field(KTabMeta::COLUMNS, serde_json::to_string(&columns)?)
            .field(KTabMeta::TABLE_NAME, table_name)
            .field(KTabMeta::CREATE_TIME, create_time)
            .field(KTabMeta::TABLE_COMMENT, table_comment)
            .field(KTabMeta::REAL_TABLE, *real_table);

        self.conn().await?.exec(omit_sql).await?;
        self.conn().await?.exec(insert_sql).await?;

        Ok(KTabOverwriteMetaRsp {})
    }

    async fn ktab_overwrite_row(
        &self,
        req: KReq<KTabOverwriteRowReq>,
    ) -> chin_tools::AResult<KTabOverwriteRowRsp> {
        let emtpy_req = req.frame(());
        let KReq { body, kspace: _ } = req;

        let KTabOverwriteRowReq { row } = body;
        for ele in row {
            self.ktab_overwrite_cell(emtpy_req.frame(KTabOverwriteCellReq { cell: ele }))
                .await?;
        }

        Ok(KTabOverwriteRowRsp {})
    }

    async fn ktab_overwrite_cell(
        &self,
        req: KReq<KTabOverwriteCellReq>,
    ) -> chin_tools::AResult<KTabOverwriteCellRsp> {
        macro_rules! overwrite {
            ($table:tt, $c:expr) => {
                let csql = SqlInserter::new($table::TABLE)
                    .field($table::TABLE_ID, &$c.table_id)
                    .field($table::COL_IDX, $c.col_idx)
                    .field($table::ROW_IDX, $c.row_idx)
                    .field($table::INSERT_TIME, &$c.insert_time);
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
            KTabCell::String(c) => {
                overwrite!(KTabCellStr1024, c);
            }
            KTabCell::Text(c) => {
                overwrite!(KTabCellText, c);
            }
            KTabCell::Integer(c) => {
                overwrite!(KTabCellInteger, c);
            }
            KTabCell::Date(c) => {
                overwrite!(KTabCellDate, c);
            }
        }

        Ok(KTabOverwriteCellRsp {})
    }

    async fn ktab_query_row(
        &self,
        req: KReq<KTabQueryRowReq>,
    ) -> chin_tools::AResult<KTabQueryRowRsp> {
        let mut cells = vec![];
        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader = SqlReader::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::ROW_IDX, req.row_index),
                    Wheres::equal($sub_table::TABLE_ID, req.table_id.as_str()),
                    Wheres::is_null($sub_table::DELETE_TIME),
                ]));

                let data: Vec<KTabCell> = self.conn().await?
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

        extend_cells!(KTabCellStr1024);
        extend_cells!(KTabCellDate);
        extend_cells!(KTabCellInteger);
        extend_cells!(KTabCellText);

        Ok(KTabQueryRowRsp { row: cells })
    }

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabQueryTableMetaReq>,
    ) -> chin_tools::AResult<KTabQueryTableMetaRsp> {
        let ssb = SqlReader::read_all(KTabMeta::TABLE).r#where(Wheres::and([
            Wheres::equal(KTabMeta::TABLE, &req.table_name),
            Wheres::is_null(KTabMeta::DELETE_TIME),
        ]));
        let meta = self.conn().await?
            .qry_one(
                ssb,
                |row| {
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
                },
                true,
            )
            .await?;

        Ok(KTabQueryTableMetaRsp { meta })
    }

    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabQueryTableDataReq>,
    ) -> chin_tools::AResult<KTabQueryTableDataRsp> {
        let mut cells = vec![];
        macro_rules! extend_cells {
            ($sub_table:tt) => {
                let reader = SqlReader::read_all($sub_table::TABLE).r#where(Wheres::and([
                    Wheres::equal($sub_table::TABLE_ID, req.table_id.as_str()),
                    Wheres::is_null($sub_table::DELETE_TIME),
                ]));

                let data: Vec<KTabCell> = self.conn().await?
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

        extend_cells!(KTabCellStr1024);
        extend_cells!(KTabCellDate);
        extend_cells!(KTabCellInteger);
        extend_cells!(KTabCellText);

        Ok(KTabQueryTableDataRsp { cells })
    }

    async fn ensure_ktab_tables(&self) -> chin_tools::EResult {
        self.conn().await?.exec(KTabCellDate::schema(self.db_type())).await?;
        self.conn().await?.exec(KTabCellInteger::schema(self.db_type())).await?;
        self.conn().await?.exec(KTabCellStr1024::schema(self.db_type())).await?;
        self.conn().await?.exec(KTabCellText::schema(self.db_type())).await?;
        self.conn().await?.exec(KTabMeta::schema(self.db_type())).await?;

        Ok(())
    }
}
