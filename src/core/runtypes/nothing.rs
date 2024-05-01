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

    pub fn to_boolean(&self) -> bool {
        false
    }
}

// implement arithmetics
impl Nothing {
    pub fn add(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn substract(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn mulitply(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn divide(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn greater_than(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::boolean(false)
    }
    pub fn less_than(&self, _operand: RuntimeType, _scopes: &ScopesStack) -> RuntimeType {
        RuntimeType::boolean(false)
    }
}
