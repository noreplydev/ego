#[derive(Debug, Clone)]
pub struct RnNumber {
    val: usize,
}

impl RnNumber {
    pub fn new(value: usize) -> RnNumber {
        RnNumber { val: value }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}
