use super::{block::Block, group::Group, identifier::Identifier};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    //pub type: String,
    pub identifier: Identifier,
    pub parameters: Group,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl FunctionDeclaration {
    pub fn new(
        identifier: Identifier,
        parameters: Group,
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
