use super::{block::Block, identifier::Identifier};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    //pub type: String,
    pub identifier: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl FunctionDeclaration {
    pub fn new(
        identifier: Identifier,
        parameters: Vec<Identifier>,
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
