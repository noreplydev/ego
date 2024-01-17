pub struct AstTree {
    pub root: AstToken,
}

impl AstTree {
    pub fn new() -> AstTree {
        AstTree {
            root: AstToken::new(),
        }
    }
}

pub struct AstToken {}

impl AstToken {
    pub fn new() -> AstToken {
        AstToken {}
    }
}
