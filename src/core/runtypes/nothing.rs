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
    pub fn add(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn substract(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn mulitply(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn divide(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::nothing()
    }
    pub fn greater_than(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::boolean(false)
    }
    pub fn less_than(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::boolean(false)
    }
    pub fn greater_than_or_equal(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::boolean(true)
    }
    pub fn less_than_or_equal(&self, _operand: RuntimeType) -> RuntimeType {
        RuntimeType::boolean(true)
    }
    pub fn not_equal(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            _ => RuntimeType::boolean(true),
        }
    }
    pub fn equal(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            _ => RuntimeType::boolean(false),
        }
    }
    pub fn or(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            _ => RuntimeType::boolean(true),
        }
    }
    pub fn and(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            _ => RuntimeType::boolean(false),
        }
    }
}
