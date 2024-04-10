use crate::{
    ast::{block::Block, group::Group},
    runtime::ScopesStack,
};

use super::{identifier::RnIdentifier, RuntimeType};

#[derive(Debug, Clone)]
pub struct RnFunction {
    pub identifier: RnIdentifier,
    pub parameters: Group,
    pub body: Block,
    pub at: usize,
    pub line: usize,
}

impl RnFunction {
    pub fn new(
        identifier: RnIdentifier,
        parameters: Group,
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
}

// implement arithmetics
impl RnFunction {
    pub fn add(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(s) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(b) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(i) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn substract(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn mulitply(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn divide(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
}
