use std::fmt::{self};

use crate::{
    ast::{block::Block, group::Group},
    core::error::ErrorType,
    runtime::ScopesStack,
};

use super::{
    boolean::RnBoolean,
    function::RnFunction,
    identifier::RnIdentifier,
    nothing::Nothing,
    number::RnNumber,
    string::RnString,
    traits::{arithmetic::Arithmetic, print::Print},
};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Nothing(Nothing),
    RnString(RnString),
    RnNumber(RnNumber),
    RnIdentifier(RnIdentifier),
    RnBoolean(RnBoolean),
    RnFunction(RnFunction),
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

    // has ast nodes dependencies
    // in its fields
    pub fn function(
        identifier: String,
        parameters: Group,
        body: Block,
        at: usize,
        line: usize,
    ) -> RuntimeType {
        let identifier = RnIdentifier::new(identifier);
        RuntimeType::RnFunction(RnFunction::new(identifier, parameters, body, at, line))
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeType::Nothing(nothing) => nothing.to_string(),
            RuntimeType::RnString(rn_string) => rn_string.to_string(),
            RuntimeType::RnNumber(rn_number) => rn_number.to_string(),
            RuntimeType::RnIdentifier(rn_number) => rn_number.to_string(),
            RuntimeType::RnBoolean(rn_boolean) => rn_boolean.to_string(),
            RuntimeType::RnFunction(rn_function) => rn_function.to_string(),
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
            RuntimeType::RnFunction(_) => write!(f, "RnFunction"),
        }
    }
}

// Traits defined for runtime types
impl Print for RuntimeType {
    fn print(&self, scopes: &ScopesStack) -> String {
        match self {
            RuntimeType::Nothing(t) => t.to_string(),
            RuntimeType::RnString(t) => t.to_string(),
            RuntimeType::RnNumber(t) => t.to_string(),
            RuntimeType::RnBoolean(t) => t.to_string(),
            RuntimeType::RnIdentifier(t) => t.resolve(scopes).to_string(),
            RuntimeType::RnFunction(t) => t.to_string(),
        }
    }
}

impl Arithmetic for RuntimeType {
    fn arithmetic(
        &self,
        operator: char,
        operand: RuntimeType,
        scopes: &ScopesStack,
    ) -> Result<RuntimeType, ErrorType> {
        match self {
            RuntimeType::Nothing(v) => match operator {
                '+' => Ok(v.add(operand, scopes)),
                '-' => Ok(v.substract(operand, scopes)),
                '*' => Ok(v.mulitply(operand, scopes)),
                '/' => Ok(v.divide(operand, scopes)),
                _ => Err(ErrorType::UnknownArithmeticOperator),
            },
            RuntimeType::RnNumber(v) => match operator {
                '+' => Ok(v.add(operand, scopes)),
                '-' => Ok(v.substract(operand, scopes)),
                '*' => Ok(v.mulitply(operand, scopes)),
                '/' => Ok(v.divide(operand, scopes)),
                _ => Err(ErrorType::UnknownArithmeticOperator),
            },
            RuntimeType::RnString(v) => match operator {
                '+' => Ok(v.add(operand, scopes)),
                '-' => Ok(v.substract(operand, scopes)),
                '*' => Ok(v.mulitply(operand, scopes)),
                '/' => Ok(v.divide(operand, scopes)),
                _ => Err(ErrorType::UnknownArithmeticOperator),
            },
            RuntimeType::RnBoolean(v) => match operator {
                '+' => Ok(v.add(operand, scopes)),
                '-' => Ok(v.substract(operand, scopes)),
                '*' => Ok(v.mulitply(operand, scopes)),
                '/' => Ok(v.divide(operand, scopes)),
                _ => Err(ErrorType::UnknownArithmeticOperator),
            },
            // RuntimeType::RnIdentifier(t) => t.resolve(scopes).to_string()
            _ => Err(ErrorType::UnknownArithmeticOperator),
        }
    }
}
