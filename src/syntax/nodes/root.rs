use std::fmt;

use super::AstNodeType;

/* AST TREE */
#[derive(Debug, Clone)]
pub struct AstTree {
    pub children: Vec<AstNodeType>,
}

impl AstTree {
    pub fn new() -> AstTree {
        AstTree { children: vec![] }
    }
}

impl fmt::Display for AstTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nAST Tree \n {:#?}", self.children)
    }
}
