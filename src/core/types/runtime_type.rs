use std::fmt;

use super::{boolean::RnBoolean, identifier::RnIdentifier, number::RnNumber, string::RnString};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Nothing(Nothing),
    RnString(RnString),
    RnNumber(RnNumber),
    RnIdentifier(RnIdentifier),
    RnBoolean(RnBoolean),
}

impl RuntimeType {
    pub fn nothing() -> RuntimeType {
        RuntimeType::Nothing(Nothing::new())
    }

    pub fn string(value: String) -> RuntimeType {
        RuntimeType::RnString(RnString::new(value))
    }

    pub fn number(value: i64) -> RuntimeType {
        RuntimeType::RnNumber(RnNumber::new(value))
    }

    pub fn identifier(value: String) -> RuntimeType {
        RuntimeType::RnIdentifier(RnIdentifier::new(value))
    }

    pub fn boolean(value: bool) -> RuntimeType {
        RuntimeType::RnBoolean(RnBoolean::new(value))
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeType::Nothing(nothing) => nothing.to_string(),
            RuntimeType::RnString(rn_string) => rn_string.to_string(),
            RuntimeType::RnNumber(rn_number) => rn_number.to_string(),
            RuntimeType::RnIdentifier(rn_number) => rn_number.to_string(),
            RuntimeType::RnBoolean(rn_boolean) => rn_boolean.to_string(),
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
            RuntimeType::RnBoolean(_) => write!(f, "RnBoolean"),
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
