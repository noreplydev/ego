use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct RnBoolean {
    val: bool,
}

impl RnBoolean {
    pub fn new(value: bool) -> RnBoolean {
        RnBoolean { val: value }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }

    pub fn to_boolean(&self) -> bool {
        self.val
    }
}

// implement arithmetics
impl RnBoolean {
    pub fn add(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + true -> nothing
            RuntimeType::RnString(s) => {
                // true + "world" -> "trueworld"
                RuntimeType::string(format!("{}{}", self.to_string(), s.to_string()), true)
            }
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(_) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn substract(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(_) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn mulitply(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(_) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn divide(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(_) => RuntimeType::nothing(),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn greater_than(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn greater_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(v) => RuntimeType::boolean(self.val == v.val),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(v) => RuntimeType::boolean(self.val == v.val),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn not_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(_) => RuntimeType::boolean(true),
            RuntimeType::RnBoolean(v) => RuntimeType::boolean(self.val != v.val),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(true),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(true),
        }
    }
    pub fn equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(v) => RuntimeType::boolean(self.val == v.val),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn or(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            _ => RuntimeType::boolean(true),
        }
    }
    pub fn and(&self, _operand: RuntimeType) -> RuntimeType {
        match _operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            _ => RuntimeType::boolean(true),
        }
    }
}
