use chin_tools::AResult;

pub trait ROSwap<T> {
    fn swap(self) -> T;
}

impl<E> ROSwap<Option<AResult<E>>> for AResult<Option<E>> {
    fn swap(self) -> Option<AResult<E>> {
        match self {
            Ok(ok) => match ok {
                Some(e) => Some(Ok(e)),
                None => Some(Err(anyhow::anyhow!("Empty result"))),
            },
            Err(err) => Some(Err(err)),
        }
    }
}

impl<E> ROSwap<AResult<Option<E>>> for Option<AResult<E>> {
    fn swap(self) -> AResult<Option<E>> {
        match self {
            Some(result) => match result {
                Ok(ok) => Ok(Some(ok)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }
}

impl<E> ROSwap<AResult<Vec<E>>> for Vec<AResult<E>> {
    fn swap(self) -> AResult<Vec<E>> {
        self.into_iter().map(|e| e).collect()
    }
}

#[macro_export]
macro_rules! flatten_result2 {
    ($result:expr) => {
        match $result {
            Ok(Ok(t)) => Ok(t),
            Ok(Err(err)) => Err(anyhow::anyhow!(err)),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    };
}

#[macro_export]
macro_rules! flatten_result3 {
    ($result:expr) => {
        match $result {
            Ok(Ok(Ok(t))) => Ok(t),
            Ok(Ok(Err(err))) => Err(anyhow::anyhow!(err)),
            Ok(Err(err)) => Err(anyhow::anyhow!(err)),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    };
}
