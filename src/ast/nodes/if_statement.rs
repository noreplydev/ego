use super::{block::Block, group::Group};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Group,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl IfStatement {
    pub fn new(condition: Group, body: Block, at: usize, line: usize) -> IfStatement {
        IfStatement {
            condition,
            body,
            at,
            line,
        }
    }
}
