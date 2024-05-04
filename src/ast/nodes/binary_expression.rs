use super::Expression;

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub at: usize,
    pub line: usize,
}

impl BinaryExpression {
    pub fn new(
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        at: usize,
        line: usize,
    ) -> BinaryExpression {
        BinaryExpression {
            operator,
            left,
            right,
            at,
            line,
        }
    }
}
