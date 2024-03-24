use super::{block::Block, identifier::IdentifierNode, Expression};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub parameters: Vec<Option<Expression>>,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl FunctionDeclaration {
    pub fn new(
        identifier: IdentifierNode,
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
