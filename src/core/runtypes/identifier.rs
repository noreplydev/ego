use super::RuntimeType;
use crate::runtime::ScopesStack;

#[derive(Debug, Clone)]
pub struct RnIdentifier {
    val: String,
}

impl RnIdentifier {
    pub fn new(value: String) -> RnIdentifier {
        RnIdentifier { val: value }
    }

    pub fn to_string(&self) -> String {
        self.val.clone()
    }

    pub fn to_boolean(&self) -> bool {
        false
    }

    pub fn resolve(&self, scopes: &ScopesStack) -> RuntimeType {
        let value_runtype = scopes.get_identifier_value(&self.val);
        if let Some(value) = value_runtype {
            value.clone()
        } else {
            RuntimeType::nothing()
        }
    }
}
