use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct RnNumber {
    pub val: f64,
}

impl RnNumber {
    pub fn new(value: f64) -> RnNumber {
        RnNumber { val: value }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }

    pub fn to_boolean(&self) -> bool {
        if self.val > 0.0 {
            true
        } else {
            false
        }
    }
}

// implement arithmetics
impl RnNumber {
    pub fn add(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(s) => {
                // 13 + "hello" -> "13hello"
                RuntimeType::string(format!("{}{}", self.to_string(), s.to_string()), true)
            }
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::number(self.val + n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn substract(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::number(self.val - n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn mulitply(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::number(self.val * n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn divide(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(),
            RuntimeType::RnString(_) => RuntimeType::nothing(),
            RuntimeType::RnBoolean(_) => RuntimeType::nothing(),
            RuntimeType::RnNumber(n) => RuntimeType::number(self.val / n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::nothing(),
            RuntimeType::RnFunction(_) => RuntimeType::nothing(),
        }
    }
    pub fn greater_than(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val > n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val < n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn greater_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val >= n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val <= n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn not_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(_) => RuntimeType::boolean(true),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(true),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val != n.val),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(true),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(true),
        }
    }
    pub fn equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(_) => RuntimeType::boolean(false),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(n) => RuntimeType::boolean(self.val == n.val),
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
