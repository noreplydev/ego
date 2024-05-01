use crate::{
    ast::{block::Block, group::Group, identifier::Identifier},
    runtime::ScopesStack,
};

use super::{identifier::RnIdentifier, RuntimeType};

#[derive(Debug, Clone)]
pub struct RnFunction {
    pub identifier: RnIdentifier,
    pub parameters: Vec<Identifier>,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl RnFunction {
    pub fn new(
        identifier: RnIdentifier,
        parameters: Vec<Identifier>,
        body: Block,
        at: usize,
        line: usize,
    ) -> RnFunction {
        RnFunction {
            identifier,
            parameters,
            body,
            at,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        self.identifier.to_string()
    }

    pub fn to_boolean(&self) -> bool {
        false
    }
}
