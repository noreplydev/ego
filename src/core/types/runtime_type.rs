use std::fmt;

use super::{number::RnNumber, string::RnString};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Nothing(Nothing),
    RnString(RnString),
    RnNumber(RnNumber),
}

impl RuntimeType {
    pub fn string(value: String) -> RuntimeType {
        RuntimeType::RnString(RnString::new(value))
    }

    pub fn nothing() -> RuntimeType {
        RuntimeType::Nothing(Nothing::new())
    }

    pub fn number(value: usize) -> RuntimeType {
        RuntimeType::RnNumber(RnNumber::new(value))
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeType::Nothing(nothing) => nothing.to_string(),
            RuntimeType::RnString(rn_string) => rn_string.to_string(),
            RuntimeType::RnNumber(rn_number) => rn_number.to_string(),
        }
    }
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeType::Nothing(_) => write!(f, "Nothing"),
            RuntimeType::RnString(_) => write!(f, "RnString"),
            RuntimeType::RnNumber(_) => write!(f, "RnNumber"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nothing {}
impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }

    pub fn to_string(&self) -> String {
        String::from("Nothing")
    }
}
