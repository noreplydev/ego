pub mod assignament_statement;
pub mod binary_expression;
pub mod block;
pub mod bool;
pub mod call_expression;
pub mod function_declaration;
pub mod group;
pub mod identifier;
pub mod module;
pub mod number;
pub mod string_literal;
use std::fmt;

use self::{
    assignament_statement::AssignamentNode, binary_expression::BinaryExpression, block::Block,
    bool::Bool, call_expression::CallExpressionNode, function_declaration::FunctionDeclaration,
    group::Group, identifier::Identifier, number::Number, string_literal::StringLiteral,
};

#[derive(Debug, Clone)]
pub enum AstNodeType {
    IfStatement,
    Group(Group),
    Block(Block),
    Expression(Expression),
    CallExpression(CallExpressionNode),
    AssignamentStatement(AssignamentNode),
    FunctionDeclaration(FunctionDeclaration),
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::CallExpression(node) => write!(f, "FunctionCall: {:#?}", node),
            AstNodeType::IfStatement => write!(f, "IfStatement"),
            AstNodeType::Block(_) => write!(f, "Block"),
            AstNodeType::Group(_) => write!(f, "Group"),
            AstNodeType::FunctionDeclaration(_) => write!(f, "FunctionDeclaration"),
            AstNodeType::AssignamentStatement(_) => write!(f, "AssignamentStatement"),
            AstNodeType::Expression(Expression::StringLiteral(_)) => write!(f, "StringLiteral"),
            AstNodeType::Expression(Expression::Number(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Bool(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Identifier(_)) => write!(f, "Identifier"),
            AstNodeType::Expression(Expression::BinaryExpression(_)) => {
                write!(f, "BinaryExpression")
            }
        }
    }
}

impl PartialEq for AstNodeType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

// AstNodeType::Expresssion
#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(StringLiteral),
    Number(Number),
    Bool(Bool),
    Identifier(Identifier),
    BinaryExpression(BinaryExpression),
}
