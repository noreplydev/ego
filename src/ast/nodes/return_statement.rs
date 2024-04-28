use super::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
    pub at: usize,
    pub line: usize,
}

impl ReturnStatement {
    pub fn new(value: Expression, at: usize, line: usize) -> ReturnStatement {
        ReturnStatement { value, at, line }
    }
}
