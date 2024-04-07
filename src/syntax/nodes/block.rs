use super::AstNodeType;

#[derive(Debug, Clone)]
pub struct Block {
    pub children: Vec<AstNodeType>,
}

impl Block {
    pub fn new() -> Block {
        Block { children: vec![] }
    }
    pub fn add_child(&mut self, node: AstNodeType) {
        self.children.push(node);
    }
}
