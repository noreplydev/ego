pub mod assignament_statement;
pub mod block;
pub mod bool;
pub mod call_expression;
pub mod function_declaration;
pub mod identifier;
pub mod module;
pub mod number;
pub mod string_literal;
use std::fmt;

use self::{
    assignament_statement::AssignamentNode, block::Block, bool::Bool,
    call_expression::CallExpressionNode, function_declaration::FunctionDeclaration,
    identifier::IdentifierNode, number::Number, string_literal::StringLiteral,
};

/* AstNodeType */

#[derive(Debug, Clone)]
pub enum AstNodeType {
    Root,
    Empty,
    Group,
    Block(Block),
    FunctionDeclaration(FunctionDeclaration),
    CallExpression(CallExpressionNode),
    IfStatement,
    AssignamentStatement(AssignamentNode),
    Expression(Expression),
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::Root => write!(f, "Root"),
            AstNodeType::Empty => write!(f, "Empty"),
            AstNodeType::Group => write!(f, "Group"),
            AstNodeType::CallExpression(node) => write!(f, "FunctionCall: {:#?}", node),
            AstNodeType::Block(_) => write!(f, "Block"),
            AstNodeType::FunctionDeclaration(_) => write!(f, "FunctionDeclaration"),
            AstNodeType::IfStatement => write!(f, "IfStatement"),
            AstNodeType::AssignamentStatement(_) => write!(f, "AssignamentStatement"),
            AstNodeType::Expression(Expression::StringLiteral(_)) => write!(f, "StringLiteral"),
            AstNodeType::Expression(Expression::Number(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Bool(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Identifier(_)) => write!(f, "Identifier"),
        }
    }
}

impl PartialEq for AstNodeType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

/* AstNodeType::Expresssion */
#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(StringLiteral),
    Number(Number),
    Bool(Bool),
    Identifier(IdentifierNode),
}
