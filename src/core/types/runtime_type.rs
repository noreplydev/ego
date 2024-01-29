use std::fmt;

use super::{identifier::RnIdentifier, number::RnNumber, string::RnString};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Nothing(Nothing),
    RnString(RnString),
    RnNumber(RnNumber),
    RnIdentifier(RnIdentifier),
}

impl RuntimeType {
    pub fn nothing() -> RuntimeType {
        RuntimeType::Nothing(Nothing::new())
    }

    pub fn string(value: String) -> RuntimeType {
        RuntimeType::RnString(RnString::new(value))
    }

    pub fn number(value: usize) -> RuntimeType {
        RuntimeType::RnNumber(RnNumber::new(value))
    }

    pub fn identifier(value: String) -> RuntimeType {
        RuntimeType::RnIdentifier(RnIdentifier::new(value))
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeType::Nothing(nothing) => nothing.to_string(),
            RuntimeType::RnString(rn_string) => rn_string.to_string(),
            RuntimeType::RnNumber(rn_number) => rn_number.to_string(),
            RuntimeType::RnIdentifier(rn_number) => rn_number.to_string(),
        }
    }
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeType::Nothing(_) => write!(f, "Nothing"),
            RuntimeType::RnString(_) => write!(f, "RnString"),
            RuntimeType::RnNumber(_) => write!(f, "RnNumber"),
            RuntimeType::RnIdentifier(_) => write!(f, "RnIdentifier"),
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
