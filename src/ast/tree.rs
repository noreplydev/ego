use std::fmt;

use crate::core::types::RuntimeType;

/* AST TREE */
#[derive(Debug, Clone)]
pub struct AstTree {
    pub root: AstNode,
}

impl AstTree {
    pub fn new(root_node: AstNode) -> AstTree {
        AstTree {
            root: AstNode::new(root_node.node_type, root_node.value, root_node.children),
        }
    }
}

impl fmt::Display for AstTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nAST Tree \n {}", self.root)
    }
}

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
}

#[derive(Debug, Clone, Copy)]
pub enum AstNodeType {
    Root,
    Block,
    Group,
    FunctionCall,
    IfStatement,
    VariableDeclaration,
    Expression(Expression),
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeType::Root => write!(f, "Root"),
            AstNodeType::Block => write!(f, "Block"),
            AstNodeType::Group => write!(f, "Group"),
            AstNodeType::FunctionCall => write!(f, "FunctionCall"),
            AstNodeType::IfStatement => write!(f, "IfStatement"),
            AstNodeType::VariableDeclaration => write!(f, "VariableDeclaration"),
            AstNodeType::Expression(Expression::StringLiteral) => write!(f, "StringLiteral"),
            AstNodeType::Expression(Expression::NumberLiteral) => write!(f, "Number"),
            AstNodeType::Expression(Expression::Identifier) => write!(f, "Indentifier"),
            AstNodeType::Expression(Expression::BinaryOperator) => write!(f, "BinaryOperator"),
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
    BinaryOperator,
}
