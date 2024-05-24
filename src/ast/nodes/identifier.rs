use super::Type;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub at: usize,
    pub line: usize,
    pub annotation: Option<Type>,
}

impl Identifier {
    pub fn new(name: String, at: usize, line: usize) -> Identifier {
        Identifier {
            name,
            line,
            at,
            annotation: None,
        }
    }

    pub fn set_annotation(&mut self, node_type: Option<Type>) {
        self.annotation = node_type;
    }
}
