use crate::{
    core::{error::ErrorType, runtypes::RuntimeType},
    runtime::ScopesStack,
};

pub trait Arithmetic {
    fn arithmetic(
        &self,
        operator: char,
        operand: RuntimeType,
        scopes: &ScopesStack,
    ) -> Result<RuntimeType, ErrorType>;
}
