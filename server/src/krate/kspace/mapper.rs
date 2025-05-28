use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    mapper::db::{KDbRow, KDbRowBehavier},
    MapperType,
};

use super::*;

pub(crate) trait KSpaceMapper {
    async fn read_all_kspaces(&self) -> AResult<Vec<KSpace>>;
    async fn overwrite_kspace(&self, kspace: KSpace) -> AResult<usize>;
}

impl KSpaceMapper for MapperType {
    async fn read_all_kspaces(&self) -> AResult<Vec<KSpace>> {
        todo!()
    }

    async fn overwrite_kspace(&self, kspace: KSpace) -> AResult<usize> {
        todo!()
    }
}
