use super::traits::print::Print;

#[derive(Debug, Clone)]
pub struct RnNumber {
    val: i64,
}

impl RnNumber {
    pub fn new(value: i64) -> RnNumber {
        RnNumber { val: value }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}

// Default traits implemented by runtime number
impl Print for RnNumber {
    fn print(&self) -> String {
        self.val.to_string()
    }
}
