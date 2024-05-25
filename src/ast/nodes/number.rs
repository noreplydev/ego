#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub at: usize,
    pub line: usize,
}

impl Number {
    pub fn new(value: f64, at: usize, line: usize) -> Number {
        Number { value, at, line }
    }

    pub fn from_string(value: String, at: usize, line: usize) -> Option<Number> {
        value.parse::<f64>().ok().map(|number| Number {
            value: number,
            at,
            line,
        })
    }
}
