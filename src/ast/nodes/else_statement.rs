use super::block::Block;

#[derive(Debug, Clone)]
pub struct ElseStatement {
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl ElseStatement {
    pub fn new(body: Block, at: usize, line: usize) -> ElseStatement {
        ElseStatement { body, at, line }
    }
}
