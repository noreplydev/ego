use super::AstNodeType;

/* AST TREE */
#[derive(Debug, Clone)]
pub struct Group {
    pub children: Vec<AstNodeType>,
}

impl Group {
    pub fn new() -> Group {
        Group { children: vec![] }
    }
    pub fn add_child(&mut self, node: AstNodeType) {
        self.children.push(node);
    }
}
