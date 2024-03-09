#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub at: usize,
    pub line: usize,
}

impl StringLiteral {
    pub fn new(value: String, at: usize, line: usize) -> StringLiteral {
        StringLiteral { value, at, line }
    }
}
