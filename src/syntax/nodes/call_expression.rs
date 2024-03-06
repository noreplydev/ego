use super::identifier::{self, IdentifierNode};

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub at: usize,
    pub line: usize,
    //pub arguments
}

impl CallExpressionNode {
    pub fn new(identifier: IdentifierNode, at: usize, line: usize) -> CallExpressionNode {
        CallExpressionNode {
            identifier,
            at,
            line,
        }
    }
}
