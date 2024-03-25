use super::{group::Group, identifier::IdentifierNode};

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub arguments: Group,
    pub at: usize,
    pub line: usize,
}

impl CallExpressionNode {
    pub fn new(
        identifier: IdentifierNode,
        arguments: Group,
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
