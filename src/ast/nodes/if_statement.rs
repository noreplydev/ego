use super::{block::Block, Expression};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl IfStatement {
    pub fn new(condition: Expression, body: Block, at: usize, line: usize) -> IfStatement {
        IfStatement {
            condition,
            body,
            at,
            line,
        }
    }
}
