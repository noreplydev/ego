#[derive(Debug, Clone)]
pub struct Nothing {}
impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }

    pub fn to_string(&self) -> String {
        String::from("nothing")
    }
}
