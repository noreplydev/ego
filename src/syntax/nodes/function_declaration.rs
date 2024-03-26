use super::{block::Block, identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    //pub type: String,
    pub identifier: Identifier,
    pub parameters: Vec<Option<Expression>>,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl FunctionDeclaration {
    pub fn new(
        identifier: Identifier,
        parameters: Vec<Option<Expression>>,
        body: Block,
        at: usize,
        line: usize,
    ) -> FunctionDeclaration {
        FunctionDeclaration {
            identifier,
            parameters,
            body,
            at,
            line,
        }
    }
}
