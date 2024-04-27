use super::{block::Block, Expression};

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl WhileStatement {
    pub fn new(condition: Expression, body: Block, at: usize, line: usize) -> WhileStatement {
        WhileStatement {
            condition,
            body,
            at,
            line,
        }
    }
}
