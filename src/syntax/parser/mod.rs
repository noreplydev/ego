use super::{module::ModuleAst, LexerToken};

mod expressions;
mod parse_ast;

use self::parse_ast::parse;

pub struct Module {
    module_name: String,
    tokens: Vec<LexerToken>,
}

impl Module {
    pub fn new(module_name: String, tokens: Vec<LexerToken>) -> Module {
        Module {
            module_name,
            tokens,
        }
    }

    pub fn parse(&self) -> ModuleAst {
        parse(self.tokens.clone(), &self.module_name)
    }
}
