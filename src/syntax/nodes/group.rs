use super::Expression;

#[derive(Debug, Clone)]
pub struct Group {
    pub children: Vec<Option<Expression>>,
    pub at: usize,
    pub line: usize,
}

impl Group {
    pub fn new(at: usize, line: usize) -> Group {
        Group {
            children: vec![],
            at,
            line,
        }
    }
    pub fn add_child(&mut self, node: Option<Expression>) {
        self.children.push(node);
    }
}
