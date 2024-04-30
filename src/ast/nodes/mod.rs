pub mod assignament_statement;
pub mod binary_expression;
pub mod block;
pub mod bool;
pub mod call_expression;
pub mod else_statement;
pub mod function_declaration;
pub mod group;
pub mod identifier;
pub mod if_statement;
pub mod import_statement;
pub mod module;
pub mod number;
pub mod return_statement;
pub mod string_literal;
pub mod while_statement;
use std::fmt;

use self::{
    assignament_statement::AssignamentNode, binary_expression::BinaryExpression, block::Block,
    bool::Bool, call_expression::CallExpression, else_statement::ElseStatement,
    function_declaration::FunctionDeclaration, group::Group, identifier::Identifier,
    if_statement::IfStatement, import_statement::ImportStatement, number::Number,
    return_statement::ReturnStatement, string_literal::StringLiteral,
    while_statement::WhileStatement,
};

#[derive(Debug, Clone)]
pub enum AstNodeType {
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ImportStatement(ImportStatement),
    ReturnStatement(ReturnStatement),
    ElseStatement(ElseStatement),
    Group(Group),
    Block(Block),
    Expression(Expression),
    CallExpression(CallExpression),
    AssignamentStatement(AssignamentNode),
    FunctionDeclaration(FunctionDeclaration),
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::CallExpression(node) => write!(f, "FunctionCall: {:#?}", node),
            AstNodeType::IfStatement(_) => write!(f, "IfStatement"),
            AstNodeType::ElseStatement(_) => write!(f, "ElseStatement"),
            AstNodeType::ImportStatement(_) => write!(f, "ImportStatement"),
            AstNodeType::WhileStatement(_) => write!(f, "WhileStatement"),
            AstNodeType::ReturnStatement(_) => write!(f, "ReturnStatement"),
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
