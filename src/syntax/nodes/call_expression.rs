use super::{identifier::IdentifierNode, Expression};

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub arguments: Vec<Option<Expression>>,
    pub at: usize,
    pub line: usize,
}

impl CallExpressionNode {
    pub fn new(
        identifier: IdentifierNode,
        arguments: Vec<Option<Expression>>,
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
