#[derive(Debug, Clone, PartialEq)]
pub struct IndexOutOfBoundsError {
    pub index: usize,
    pub len: usize,
}

impl std::fmt::Display for IndexOutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index {} out of bounds (len {})", self.index, self.len)
    }
}
