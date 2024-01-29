#[derive(Debug, Clone)]
pub struct RnString {
    val: String,
}

impl RnString {
    pub fn new(value: String) -> RnString {
        RnString { val: value }
    }

    pub fn to_string(&self) -> String {
        interpolate(self.val.clone())
    }
}

fn interpolate(string: String) -> String {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}
