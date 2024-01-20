use std::fmt;

/* AST TREE */
#[derive(Debug)]
pub struct AstTree {
    pub root: AstNode,
}

impl AstTree {
    pub fn new(root_node: AstNode) -> AstTree {
        AstTree {
            root: AstNode::new(root_node.token_type, root_node.value, root_node.children),
        }
    }
}

impl fmt::Display for AstTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nAST Tree \n {}", self.root)
    }
}

/* AST TOKEN */
#[derive(Debug)]
pub struct AstNode {
    pub token_type: AstTokenType,
    pub value: String,
    pub children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(token_type: AstTokenType, value: String, children: Vec<AstNode>) -> AstNode {
        AstNode {
            token_type,
            value,
            children,
        }
    }

    pub fn root() -> AstNode {
        AstNode {
            token_type: AstTokenType::Root,
            value: String::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- {}: {}", self.token_type, self.value)?;
        for child in &self.children {
            write!(f, "child: {}", child)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum AstTokenType {
    Root,
    FunctionCall,
    VariableDeclaration,
    Identifier,
    StringLiteral,
}

impl fmt::Display for AstTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstTokenType::Root => write!(f, "Root"),
            AstTokenType::FunctionCall => write!(f, "FunctionCall"),
            AstTokenType::VariableDeclaration => write!(f, "VariableDeclaration"),
            AstTokenType::Identifier => write!(f, "Identifier"),
            AstTokenType::StringLiteral => write!(f, "StringLiteral"),
        }
    }
}
