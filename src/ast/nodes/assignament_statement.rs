use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct AssignamentNode {
    //pub type: String,
    pub identifier: Identifier,
    pub init: Expression,
    pub var_type: VarType,
    pub at: usize,
    pub line: usize,
}

impl AssignamentNode {
    pub fn new(
        identifier: Identifier,
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
    None,
}
