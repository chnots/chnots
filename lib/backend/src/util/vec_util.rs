pub trait RemoveNth<T> {
    fn remove_n(&mut self, n: usize) -> Option<T>;
}

impl<T> RemoveNth<T> for Vec<T> {
    fn remove_n(&mut self, n: usize) -> Option<T> {
        if self.len() >= n {
            return None;
        }
        Some(self.remove(n - 1))
    }
}
