use super::{group::Group, identifier::Identifier};

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: Identifier,
    pub arguments: Group,
    pub at: usize,
    pub line: usize,
}

impl CallExpressionNode {
    pub fn new(
        identifier: Identifier,
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
