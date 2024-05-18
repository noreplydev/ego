use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct RnString {
    pub val: String,
    pub raw: bool,
}

impl RnString {
    pub fn new(value: String, raw: bool) -> RnString {
        RnString { val: value, raw }
    }

    pub fn to_string(&self) -> String {
        if self.raw {
            return self.val.clone();
        }

        let mut chars = self.val.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }

    pub fn to_boolean(&self) -> bool {
        if self.val.len() > 0 {
            true
        } else {
            false
        }
    }
}

// implement arithmetics
impl RnString {
    pub fn add(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::nothing(), // nothing + nothing -> nothing
            RuntimeType::RnString(s) => {
                // "hello" + "world" -> "helloworld"
                RuntimeType::string(format!("{}{}", self.to_string(), s.to_string()), true)
            }
            RuntimeType::RnBoolean(b) => {
                // "hello" + true -> "hellotrue"
                RuntimeType::string(format!("{}{}", self.to_string(), b.to_string()), true)
            }
            RuntimeType::RnNumber(n) => {
                // "hello" + 13-> "hello13"
                RuntimeType::string(format!("{}{}", self.to_string(), n.to_string()), true)
            }
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
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val.len() > v.val.len()),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val.len() < v.val.len()),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn greater_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val.len() >= v.val.len()),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn less_than_or_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val.len() <= v.val.len()),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(false),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(false),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(false),
        }
    }
    pub fn not_equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(true),
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val != v.val),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(true),
            RuntimeType::RnNumber(_) => RuntimeType::boolean(true),
            RuntimeType::RnIdentifier(_) => RuntimeType::boolean(true),
            RuntimeType::RnFunction(_) => RuntimeType::boolean(true),
        }
    }
    pub fn equal(&self, operand: RuntimeType) -> RuntimeType {
        match operand {
            RuntimeType::Nothing(_) => RuntimeType::boolean(false),
            RuntimeType::RnString(v) => RuntimeType::boolean(self.val == v.val),
            RuntimeType::RnBoolean(_) => RuntimeType::boolean(false),
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
