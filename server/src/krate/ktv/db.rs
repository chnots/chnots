use chin_tools::AResult;

use crate::mapper::db::{KDbRow, KDbRowBehavier};

use super::{mapper::KTVDeserializeMapper, *};

impl KTVDeserializeMapper for KDbRow {
    fn to_ktv(self) -> AResult<KTV> {
        let obj = KTV {
            insert_time: self.try_get(KTV::INSERT_TIME)?,
            key: self.try_get(KTV::KEY)?,
            value: self.try_get(KTV::VALUE)?,
            ttype: self.try_get(KTV::TTYPE)?,
            update_time: self.try_get(KTV::UPDATE_TIME)?,
        };
        Ok(obj)
    }
}
