#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub at: usize,
    pub line: usize,
}

impl BreakStatement {
    pub fn new(at: usize, line: usize) -> BreakStatement {
        BreakStatement { at, line }
    }
}
