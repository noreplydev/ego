use super::traits::print::Print;

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

// Default traits implemented by runtime number
impl Print for RnBoolean {
    fn print(&self) -> String {
        self.val.to_string()
    }
}
