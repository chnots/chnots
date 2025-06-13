use std::ops::Deref;

/// When there are some global data, use this.
/// e.g. in table KKV.
pub const NO_KSPACE: &str = "#NO_KSPACE#";

pub(crate) struct KImplWrapper<T: Send + Sync>(pub(crate) T);
impl<T: Send + Sync> From<T> for KImplWrapper<T> {
    fn from(value: T) -> Self {
        KImplWrapper(value)
    }
}

impl<T: Send + Sync> Deref for KImplWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<T: Send + Sync> Send for KImplWrapper<T> {}
unsafe impl<T: Send + Sync> Sync for KImplWrapper<T> {}
