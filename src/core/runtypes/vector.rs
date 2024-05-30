use super::RuntimeType;

#[derive(Debug, Clone)]
pub struct RnVector {
    elements: Vec<RuntimeType>,
}

impl RnVector {
    pub fn new(value: Vec<RuntimeType>) -> RnVector {
        RnVector { elements: value }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }

    pub fn to_boolean(&self) -> bool {
        self.val
    }
}
