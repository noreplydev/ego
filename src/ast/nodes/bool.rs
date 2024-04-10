#[derive(Debug, Clone)]
pub struct Bool {
    pub value: bool,
    pub at: usize,
    pub line: usize,
}

impl Bool {
    pub fn new(value: bool, at: usize, line: usize) -> Bool {
        Bool { value, at, line }
    }
}
