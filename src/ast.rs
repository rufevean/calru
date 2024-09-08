
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    BinaryOperation {
        operator: String,
        left: Box<AST>,
        right: Box<AST>,
    },
    Assignment {
        variable: String,
        expression: Box<AST>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub node: ASTNode,
}

impl AST {
    pub fn new(node: ASTNode) -> AST {
        AST { node }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Number(n) => write!(f, "Number({})", n),
            ASTNode::Identifier(id) => write!(f, "Identifier({})", id),
            ASTNode::BinaryOperation { operator, left, right } => {
                write!(f, "BinaryOperation({} {} {})", left, operator, right)
            }
            ASTNode::Assignment { variable, expression } => {
                write!(f, "Assignment({} = {})", variable, expression)
            }
        }
    }
}
