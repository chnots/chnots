use chin_sql::{DbType, IntoSqlSeg, PlaceHolderType, SqlReader, SqlSeg, Wheres};
use chin_tools::AResult;

pub(crate) struct TableDumpSqlBuilder<'a> {
    pub(crate) table_name: &'static str,
    start_seg: Option<Wheres<'a>>,
    end_seg: Option<Wheres<'a>>,
}

impl<'a> TableDumpSqlBuilder<'a> {
    pub(crate) fn table(table_name: &'static str) -> Self {
        Self {
            table_name,
            start_seg: None,
            end_seg: None,
        }
    }

    pub(crate) fn start(self, seg: Wheres<'a>) -> Self {
        Self {
            start_seg: Some(seg),
            ..self
        }
    }

    pub(crate) fn end(self, seg: Wheres<'a>) -> Self {
        Self {
            end_seg: Some(seg),
            ..self
        }
    }

    pub(crate) fn build(self, db_type: DbType) -> AResult<SqlSeg<'a>> {
        let sql = SqlReader::read_all(self.table_name)
            .r#where(Wheres::and([
                Wheres::if_some(self.start_seg, |e| e),
                Wheres::if_some(self.end_seg, |e| e),
            ]))
            .into_sql_seg(db_type)?;
        Ok(sql)
    }
}
