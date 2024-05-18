use crate::core::runtypes::RuntimeType;

pub fn type_of(rt: RuntimeType) -> Option<RuntimeType> {
    Some(match rt {
        RuntimeType::Nothing(_) => RuntimeType::string("nothing".to_string(), true),
        RuntimeType::RnBoolean(_) => RuntimeType::string("boolean".to_string(), true),
        RuntimeType::RnNumber(_) => RuntimeType::string("number".to_string(), true),
        RuntimeType::RnIdentifier(_) => RuntimeType::string("identifier".to_string(), true),
        RuntimeType::RnString(_) => RuntimeType::string("string".to_string(), true),
        RuntimeType::RnFunction(_) => RuntimeType::string("function".to_string(), true),
    })
}
