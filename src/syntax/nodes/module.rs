use std::fmt;

use super::AstNodeType;

/* AST TREE */
#[derive(Debug, Clone)]
pub struct ModuleAst {
    pub children: Vec<AstNodeType>,
}

impl ModuleAst {
    pub fn new() -> ModuleAst {
        ModuleAst { children: vec![] }
    }
}

impl fmt::Display for ModuleAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nModule AST\n {:#?}", self.children)
    }
}
