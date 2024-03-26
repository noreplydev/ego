use crate::syntax::{
    binary_expression::BinaryExpression, bool::Bool, AstNodeType, Expression, LexerToken,
};

// 2 + 3 * 23
pub fn parse_expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    (
        3,
        AstNodeType::Expression(Expression::BinaryExpression(BinaryExpression::new(
            '+',
            Box::new(Expression::Bool(Bool::new(true, 0, 0))),
            Box::new(Expression::Bool(Bool::new(true, 0, 0))),
            0,
            0,
        ))),
    )
}
