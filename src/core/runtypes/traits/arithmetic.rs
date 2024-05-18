use crate::core::{error::ErrorType, runtypes::RuntimeType};

pub trait Arithmetic {
    fn arithmetic(&self, operator: &str, operand: RuntimeType) -> Result<RuntimeType, ErrorType>;
}
