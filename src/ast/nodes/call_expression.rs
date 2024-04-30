use super::{group::Group, identifier::Identifier};

#[derive(Debug, Clone)]
pub struct CallExpression {
    //pub type: String,
    pub identifier: Identifier,
    pub arguments: Group,
    pub at: usize,
    pub line: usize,
}

impl CallExpression {
    pub fn new(identifier: Identifier, arguments: Group, at: usize, line: usize) -> CallExpression {
        CallExpression {
            identifier,
            arguments,
            at,
            line,
        }
    }
}
