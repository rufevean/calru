use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    List(Vec<AST>),
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
    Fetch {
        list: Box<AST>,
        index: Box<AST>,
    },
    Push {
        list: Box<AST>,
        value: Box<AST>,
    },
    Pop {
        list: Box<AST>,
    },
    Loop {
        body: Box<AST>,
    },
    Break,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Int(value) => write!(f, "Int({})", value),
            ASTNode::Float(value) => write!(f, "Float({})", value),
            ASTNode::Boolean(value) => write!(f, "Boolean({})",value),
            ASTNode::Identifier(id) => write!(f, "Identifier({})", id),
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
            ASTNode::Fetch { list, index } => {
                write!(f, "Fetch({}.fetch({}))", list, index)
            }
            ASTNode::Push { list, value } => {
                write!(f, "Push({}.push({}))", list, value)
            }
            ASTNode::Pop { list } => {
                write!(f, "Pop({}.pop())", list)
            }
            ASTNode::Loop { body } => {
                write!(f, "Loop({})", body)
            }
            ASTNode::Break => {
                write!(f, "Break")
            }
            ASTNode::List(elements) => {
                let elements_str = elements.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "List([{}])", elements_str)
            }
            ASTNode::BinaryOperation { left, right, operator } => {
                write!(f, "BinaryOperation({} {} {})", left, operator, right)
            }
        }
    }
}