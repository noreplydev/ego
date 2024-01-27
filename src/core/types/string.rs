#[derive(Debug, Clone)]
pub struct RnString {
    val: String,
}

impl RnString {
    pub fn new(value: String) -> RnString {
        RnString { val: value }
    }
}
