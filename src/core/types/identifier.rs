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
