use chin_sql::{DbType, IntoSqlSeg, PlaceHolderType, SqlSegBuilder, SqlSeg, Wheres};
use chin_tools::AResult;

pub struct TableDumpSqlBuilder<'a> {
    pub table_name: &'static str,
    start_seg: Option<Wheres<'a>>,
    end_seg: Option<Wheres<'a>>,
    ph_type: PlaceHolderType,
}

impl<'a> TableDumpSqlBuilder<'a> {
    pub fn new(
        table_name: &'static str,
        start_seg: Option<Wheres<'a>>,
        end_seg: Option<Wheres<'a>>,
        ph_type: PlaceHolderType,
    ) -> Self {
        Self {
            table_name,
            ph_type,
            start_seg,
            end_seg,
        }
    }

    pub fn build(self, db_type: DbType) -> AResult<SqlSeg<'a>> {
        let sql = SqlSegBuilder::new()
            .raw("select * from")
            .raw(self.table_name)
            .r#where(Wheres::and([
                Wheres::if_some(self.start_seg, |e| e),
                Wheres::if_some(self.end_seg, |e| e),
            ]))
            .into_sql_seg2(db_type, &mut match self.ph_type {
                PlaceHolderType::QustionMark => PlaceHolderType::QustionMark,
                PlaceHolderType::DollarNumber(_) => PlaceHolderType::DollarNumber(0),
            })?;
        Ok(sql)
    }
}
