use super::{identifier::IdentifierNode, Expression};

#[derive(Debug, Clone)]
pub struct AssignamentNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub init: Expression,
    pub var_type: VarType,
    pub at: usize,
    pub line: usize,
}

impl AssignamentNode {
    pub fn new(
        identifier: IdentifierNode,
        init: Expression,
        var_type: VarType,
        at: usize,
        line: usize,
    ) -> AssignamentNode {
        AssignamentNode {
            identifier,
            init,
            var_type,
            at,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Let,
    Const,
}
