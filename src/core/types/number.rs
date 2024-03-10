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
