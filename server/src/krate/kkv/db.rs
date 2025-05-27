use chin_tools::AResult;

use crate::mapper::db::{KDbRow, KDbRowBehavier};

use super::{mapper::KKVDeserializeMapper, *};

impl KKVDeserializeMapper for KDbRow {
    fn to_kkv(self) -> AResult<KKV> {
        let obj = KKV {
            insert_time: self.try_get(KKV::INSERT_TIME)?,
            key: self.try_get(KKV::KEY)?,
            value: self.try_get(KKV::VALUE)?,
            kind: self.try_get(KKV::KIND)?,
            update_time: self.try_get(KKV::UPDATE_TIME)?,
        };
        Ok(obj)
    }
}
