#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
    pub at: usize,
    pub line: usize,
}

impl IdentifierNode {
    pub fn new(name: String, at: usize, line: usize) -> IdentifierNode {
        IdentifierNode { name, line, at }
    }
}
