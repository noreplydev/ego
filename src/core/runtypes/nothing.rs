use crate::runtime::ScopesStack;

use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct Nothing {}
impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }

    pub fn to_string(&self) -> String {
        String::from("nothing")
    }
}

// implement arithmetics
impl Nothing {
    pub fn add(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn substract(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn mulitply(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn divide(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
}
