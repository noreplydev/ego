#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub at: usize,
    pub line: usize,
}

impl Identifier {
    pub fn new(name: String, at: usize, line: usize) -> Identifier {
        Identifier { name, line, at }
    }
}
