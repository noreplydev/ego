use super::traits::print::Print;

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
}

// Default traits implemented by runtime number
impl Print for RnIdentifier {
    fn print(&self) -> String {
        self.val.to_string()
    }
}
