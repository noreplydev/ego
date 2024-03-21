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
}
