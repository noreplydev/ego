use std::fmt;

use super::string::RnString;

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Nothing(Nothing),
    RnString(RnString),
}

impl RuntimeType {
    pub fn string(value: String) -> RuntimeType {
        RuntimeType::RnString(RnString::new(value))
    }

    pub fn nothing() -> RuntimeType {
        RuntimeType::Nothing(Nothing::new())
    }

    pub fn to_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeType::Nothing(_) => write!(f, "Nothing"),
            RuntimeType::RnString(_) => write!(f, "RnString"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nothing {}
impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }
}
