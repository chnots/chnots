use chin_tools::sql::{PlaceHolderType, SqlSeg, SqlSegBuilder, Wheres};

pub struct TableDumpSql<'a> {
    pub table_name: String,
    start_seg: Option<Wheres<'a>>,
    end_seg: Option<Wheres<'a>>,
    ph_type: PlaceHolderType,
}

impl<'a> TableDumpSql<'a> {
    pub fn new(
        table_name: String,
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

    pub fn build(self) -> Option<SqlSeg<'a>> {
        SqlSegBuilder::new()
            .raw("select * from")
            .raw_owned(self.table_name)
            .r#where(Wheres::and([
                Wheres::if_some(self.start_seg, |e| e),
                Wheres::if_some(self.end_seg, |e| e),
            ]))
            .build(&mut match self.ph_type {
                PlaceHolderType::QustionMark => PlaceHolderType::QustionMark,
                PlaceHolderType::DollarNumber(_) => PlaceHolderType::DollarNumber(0),
            })
    }
}
