use std::fmt;

use super::AstNodeType;

#[derive(Debug, Clone)]
pub struct ModuleAst {
    pub module_name: String,
    pub children: Vec<AstNodeType>,
}

impl ModuleAst {
    pub fn new(module_name: &str) -> ModuleAst {
        ModuleAst {
            module_name: module_name.to_string(),
            children: vec![],
        }
    }
    pub fn add_child(&mut self, node: AstNodeType) {
        self.children.push(node);
    }
}

impl fmt::Display for ModuleAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nModule AST\n {:#?}", self.children)
    }
}
