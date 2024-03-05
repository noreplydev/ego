use super::identifier::IdentifierNode;

#[derive(Debug, Clone)]
pub struct CallExpressionNode {
    //pub type: String,
    pub identifier: IdentifierNode,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    //pub arguments
}
