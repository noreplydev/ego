#[derive(Debug, Clone)]
pub struct Number {
    pub value: i64,
    pub at: usize,
    pub line: usize,
}

impl Number {
    pub fn new(value: i64, at: usize, line: usize) -> Number {
        Number { value, at, line }
    }
}
