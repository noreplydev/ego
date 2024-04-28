#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub module: Vec<String>,
    pub members: Vec<String>,
    pub at: usize,
    pub line: usize,
}

impl ImportStatement {
    pub fn new(
        module: Vec<String>,
        members: Vec<String>,
        at: usize,
        line: usize,
    ) -> ImportStatement {
        ImportStatement {
            module,
            members,
            at,
            line,
        }
    }
}
