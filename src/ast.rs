use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    List(Vec<AST>), // Add list variant
    BinaryOperation {
        operator: String,
        left: Box<AST>,
        right: Box<AST>,
    },
    Assignment {
        variable: String,
        expression: Box<AST>,
    },
    Print(Box<AST>),
    If {
        condition: Box<AST>,
        then_branch: Box<AST>,
        else_branch: Option<Box<AST>>,
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
            ASTNode::Int(n) => write!(f, "Int({})", n),
            ASTNode::Float(n) => write!(f, "Float({})", n),
            ASTNode::Boolean(b) => write!(f, "Boolean({})", b),
            ASTNode::Identifier(id) => write!(f, "Identifier({})", id),
            ASTNode::List(elements) => {
                let elements_str: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "List([{}])", elements_str.join(", "))
            }
            ASTNode::BinaryOperation { operator, left, right } => {
                write!(f, "BinaryOperation({} {} {})", left, operator, right)
            }
            ASTNode::Assignment { variable, expression } => {
                write!(f, "Assignment({} = {})", variable, expression)
            }
            ASTNode::Print(expression) => {
                write!(f, "Print({})", expression)
            }
            ASTNode::If { condition, then_branch, else_branch } => {
                write!(f, "If({} then {} else {})", condition, then_branch, 
                    if let Some(else_branch) = else_branch { else_branch.to_string() } else { "None".to_string() })
            }
        }
    }
}