use super::{block::Block, else_statement::ElseStatement, Expression};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
    pub else_node: Option<ElseStatement>,
    pub at: usize,
    pub line: usize,
}

impl IfStatement {
    pub fn new(
        condition: Expression,
        body: Block,
        else_node: Option<ElseStatement>,
        at: usize,
        line: usize,
    ) -> IfStatement {
        IfStatement {
            condition,
            body,
            else_node,
            at,
            line,
        }
    }
}
