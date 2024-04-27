use crate::runtime::ScopesStack;

use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct RnString {
    val: String,
}

impl RnString {
    pub fn new(value: String) -> RnString {
        RnString { val: value }
    }

    pub fn to_string(&self) -> String {
        interpolate(self.val.clone())
    }

    pub fn to_boolean(&self) -> bool {
        if self.val.len() > 0 {
            true
        } else {
            false
        }
    }
}

fn interpolate(string: String) -> String {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}

// implement arithmetics
impl RnString {
    pub fn add(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(s) => {
                // "hello" + "world" -> "helloworld"
                RuntimeType::string(format!("\"{}{}\"", self.to_string(), s.to_string()))
            }
            RuntimeType::RnBoolean(b) => {
                // "hello" + true -> "hellotrue"
                RuntimeType::string(format!("\"{}{}\"", self.to_string(), b.to_string()))
            }
            RuntimeType::RnNumber(n) => {
                // "hello" + 13-> "hello13"
                RuntimeType::string(format!("\"{}{}\"", self.to_string(), n.to_string()))
            }
            RuntimeType::RnIdentifier(i) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn substract(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn mulitply(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn divide(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn greater_than(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than(&self, operand: RuntimeType, scopes: &ScopesStack) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
}
