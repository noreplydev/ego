pub mod assignament_statement;
pub mod binary_expression;
pub mod block;
pub mod bool;
pub mod break_statement;
pub mod call_expression;
pub mod else_statement;
pub mod function_declaration;
pub mod group;
pub mod identifier;
pub mod if_statement;
pub mod import_statement;
pub mod module;
pub mod nothing;
pub mod number;
pub mod return_statement;
pub mod string_literal;
pub mod vector;
pub mod while_statement;
use std::fmt;

use self::{
    assignament_statement::AssignamentNode, binary_expression::BinaryExpression, block::Block,
    bool::Bool, break_statement::BreakStatement, call_expression::CallExpression,
    else_statement::ElseStatement, function_declaration::FunctionDeclaration, group::Group,
    identifier::Identifier, if_statement::IfStatement, import_statement::ImportStatement,
    nothing::Nothing, number::Number, return_statement::ReturnStatement,
    string_literal::StringLiteral, vector::Vector, while_statement::WhileStatement,
};

#[derive(Debug, Clone)]
pub enum AstNodeType {
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ImportStatement(ImportStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ElseStatement(ElseStatement),
    Group(Group),
    Vector(Vector),
    Block(Block),
    Expression(Expression),
    AssignamentStatement(AssignamentNode),
    FunctionDeclaration(FunctionDeclaration),
}

impl AstNodeType {
    pub fn at(&self) -> usize {
        match self {
            AstNodeType::IfStatement(v) => v.at,
            AstNodeType::WhileStatement(v) => v.at,
            AstNodeType::ImportStatement(v) => v.at,
            AstNodeType::ReturnStatement(v) => v.at,
            AstNodeType::BreakStatement(v) => v.at,
            AstNodeType::ElseStatement(v) => v.at,
            AstNodeType::Group(v) => v.at,
            AstNodeType::Vector(v) => v.at,
            AstNodeType::Block(_v) => 0,
            AstNodeType::Expression(_v) => 0,
            AstNodeType::AssignamentStatement(v) => v.at,
            AstNodeType::FunctionDeclaration(v) => v.at,
        }
    }

    pub fn line(&self) -> usize {
        match self {
            AstNodeType::IfStatement(v) => v.line,
            AstNodeType::WhileStatement(v) => v.line,
            AstNodeType::ImportStatement(v) => v.line,
            AstNodeType::ReturnStatement(v) => v.line,
            AstNodeType::BreakStatement(v) => v.line,
            AstNodeType::ElseStatement(v) => v.line,
            AstNodeType::Group(v) => v.line,
            AstNodeType::Vector(v) => v.line,
            AstNodeType::Block(_v) => 0,
            AstNodeType::Expression(_v) => 0,
            AstNodeType::AssignamentStatement(v) => v.line,
            AstNodeType::FunctionDeclaration(v) => v.line,
        }
    }
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::IfStatement(_) => write!(f, "IfStatement"),
            AstNodeType::ElseStatement(_) => write!(f, "ElseStatement"),
            AstNodeType::ImportStatement(_) => write!(f, "ImportStatement"),
            AstNodeType::WhileStatement(_) => write!(f, "WhileStatement"),
            AstNodeType::ReturnStatement(_) => write!(f, "ReturnStatement"),
            AstNodeType::BreakStatement(_) => write!(f, "BreakStatement"),
            AstNodeType::Block(_) => write!(f, "Block"),
            AstNodeType::Group(_) => write!(f, "Group"),
            AstNodeType::Vector(_) => write!(f, "Vector"),
            AstNodeType::FunctionDeclaration(_) => write!(f, "FunctionDeclaration"),
            AstNodeType::AssignamentStatement(_) => write!(f, "AssignamentStatement"),
            AstNodeType::Expression(Expression::StringLiteral(_)) => write!(f, "StringLiteral"),
            AstNodeType::Expression(Expression::Number(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Bool(_)) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Identifier(_)) => write!(f, "Identifier"),
            AstNodeType::Expression(Expression::Nothing(_)) => write!(f, "Nothing"),
            AstNodeType::Expression(Expression::CallExpression(node)) => {
                write!(f, "CallExpression: {:#?}", node)
            }
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
    CallExpression(CallExpression),
    Nothing(Nothing),
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    String,
    Number,
    Bool,
    Nothing,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::String => write!(f, "string"),
            Type::Number => write!(f, "number"),
            Type::Bool => write!(f, "bool"),
            Type::Nothing => write!(f, "nothing"),
        }
    }
}
