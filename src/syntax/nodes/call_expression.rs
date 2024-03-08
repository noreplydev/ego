use super::{
    identifier::{self, IdentifierNode},
    AstNodeType,
};

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub arguments: Vec<Option<AstNodeType>>,
    pub at: usize,
    pub line: usize,
}

impl CallExpressionNode {
    pub fn new(
        identifier: IdentifierNode,
        arguments: Vec<Option<AstNodeType>>,
        at: usize,
        line: usize,
    ) -> CallExpressionNode {
        CallExpressionNode {
            identifier,
            arguments,
            at,
            line,
        }
    }
}
