#[derive(Debug, Clone)]
pub struct Nothing {
    pub at: usize,
    pub line: usize,
}

impl Nothing {
    pub fn new(at: usize, line: usize) -> Nothing {
        Nothing { at, line }
    }
}
