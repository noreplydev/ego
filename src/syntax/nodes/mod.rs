pub mod bool;
pub mod call_expression;
pub mod identifier;
pub mod module;
pub mod number;
pub mod string_literal;
use std::fmt;

use self::{
    bool::Bool, call_expression::CallExpressionNode, number::Number, string_literal::StringLiteral,
};

/*
/* AST TOKEN */
#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub value: RuntimeType,
    pub children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(node_type: AstNodeType, value: RuntimeType, children: Vec<AstNode>) -> AstNode {
        AstNode {
            node_type,
            value,
            children,
        }
    }

    pub fn root() -> AstNode {
        AstNode {
            node_type: AstNodeType::Root,
            value: RuntimeType::nothing(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- {}: {}", self.node_type, self.value)?;
        for child in &self.children {
            write!(f, "child: {}", child)?;
        }

        Ok(())
    }
} */

#[derive(Debug, Clone)]
pub enum AstNodeType {
    Root,
    Empty,
    Block,
    Group,
    FunctionCall(CallExpressionNode),
    StringLiteral(StringLiteral),
    Number(Number),
    Bool(Bool),
    IfStatement,
    VariableDeclaration,
    Expression(Expression),
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::Root => write!(f, "Root"),
            AstNodeType::Empty => write!(f, "Empty"),
            AstNodeType::Block => write!(f, "Block"),
            AstNodeType::Group => write!(f, "Group"),
            AstNodeType::FunctionCall(node) => write!(f, "FunctionCall: {:#?}", node),
            AstNodeType::StringLiteral(node) => write!(f, "StringLiteral: {:#?}", node),
            AstNodeType::Number(node) => write!(f, "NumberLiteral: {:#?}", node),
            AstNodeType::Bool(node) => write!(f, "Bool: {:#?}", node),
            AstNodeType::IfStatement => write!(f, "IfStatement"),
            AstNodeType::VariableDeclaration => write!(f, "VariableDeclaration"),
            AstNodeType::Expression(Expression::StringLiteral) => write!(f, "StringLiteral"),
            AstNodeType::Expression(Expression::NumberLiteral) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Identifier) => write!(f, "Indentifier"),
            AstNodeType::Expression(Expression::Binary(BinaryOperator::AddOperator)) => {
                write!(f, "BinaryOperator")
            }
            AstNodeType::Expression(Expression::Binary(BinaryOperator::SubtractOperator)) => {
                write!(f, "SubtractOperator")
            }
            AstNodeType::Expression(Expression::Binary(BinaryOperator::MultiplyOperator)) => {
                write!(f, "MultiplyOperator")
            }
            AstNodeType::Expression(Expression::Binary(BinaryOperator::DivisionOperator)) => {
                write!(f, "DivisionOperator")
            }
        }
    }
}

impl PartialEq for AstNodeType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Expression {
    StringLiteral,
    NumberLiteral,
    Identifier,
    Binary(BinaryOperator),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    AddOperator,
    SubtractOperator,
    DivisionOperator,
    MultiplyOperator,
}
