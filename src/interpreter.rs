
use crate::symbol_table::{SymbolTable, SymbolType, SymbolValue};
use crate::ast::{AST, ASTNode};
use std::fmt;

pub struct Interpreter {
    symbol_table: SymbolTable,
}

impl Interpreter {
pub fn new(symbol_table: SymbolTable) -> Self {
    Interpreter {
        symbol_table,
    }
}

    pub fn run(&mut self, statements: Vec<AST>) -> Result<(), String> {
        for statement in statements {
            self.execute_statement(&statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, ast: &AST) -> Result<(), String> {
        match &ast.node {
            ASTNode::Assignment { variable, expression } => {
                let value = self.evaluate_expression(expression)?;
                let symbol_type = self.symbol_table.lookup(variable)
                    .ok_or_else(|| format!("Variable '{}' not found.", variable))?
                    .symbol_type.clone();

                if self.infer_type(&AST::new(ASTNode::Assignment { variable: variable.clone(), expression: expression.clone() }))? != symbol_type {
                    return Err(format!(
                        "Type mismatch: cannot assign value of type {:?} to variable of type {:?}.",
                        self.infer_type(&AST::new(ASTNode::Assignment { variable: variable.clone(), expression: expression.clone() }))?,
                        symbol_type
                    ));
                }

                self.symbol_table.insert(variable.clone(), symbol_type, value)?;
            },
            ASTNode::Print(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{:?}", value); // Use `{:?}` for debug output
            },
            _ => return Err(format!("Unsupported statement {:?}", ast.node)),
        }
        Ok(())
    }

    fn evaluate_expression(&self, expression: &AST) -> Result<SymbolValue, String> {
        match &expression.node {
            ASTNode::Int(value) => Ok(SymbolValue::Int(*value)),
            ASTNode::Float(value) => Ok(SymbolValue::Float(*value)),
            ASTNode::Identifier(id) => {
                let symbol = self.symbol_table.lookup(id)
                    .ok_or_else(|| format!("Variable '{}' not found.", id))?;
                Ok(symbol.value.clone())
            },
            ASTNode::BinaryOperation { left, right, operator } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;

                match (left_value, right_value) {
                    (SymbolValue::Int(left_val), SymbolValue::Int(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => left_val + right_val,
                            "-" => left_val - right_val,
                            "*" => left_val * right_val,
                            "/" => left_val / right_val,
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(SymbolValue::Int(result))
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => left_val + right_val,
                            "-" => left_val - right_val,
                            "*" => left_val * right_val,
                            "/" => left_val / right_val,
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                    (SymbolValue::Int(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => (left_val as f64) + right_val,
                            "-" => (left_val as f64) - right_val,
                            "*" => (left_val as f64) * right_val,
                            "/" => (left_val as f64) / right_val,
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Int(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => left_val + (right_val as f64),
                            "-" => left_val - (right_val as f64),
                            "*" => left_val * (right_val as f64),
                            "/" => left_val / (right_val as f64),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                }
            },
            _ => Err(format!("Cannot evaluate expression node {:?}", expression.node)),
        }
    }

    fn infer_type(&self, node: &AST) -> Result<SymbolType, String> {
        match &node.node {
            ASTNode::Int(_) => Ok(SymbolType::Int),
            ASTNode::Float(_) => Ok(SymbolType::Float),
            ASTNode::Identifier(id) => {
                self.symbol_table.lookup(id)
                    .map(|symbol| symbol.symbol_type.clone())
                    .ok_or_else(|| format!("Undeclared variable '{}' at position.", id))
            },
            ASTNode::BinaryOperation { left, right, .. } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                if left_type == right_type {
                    Ok(left_type)
                } else {
                    Err(format!(
                        "Type mismatch in binary operation. Expected {:?}, found {:?}.",
                        left_type, right_type
                    ))
                }
            },
            ASTNode::Assignment { expression, .. } => self.infer_type(expression),
            ASTNode::Print(expression) => self.infer_type(expression),
        }
    }
}

impl fmt::Display for SymbolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolValue::Int(value) => write!(f, "{}", value),
            SymbolValue::Float(value) => write!(f, "{}", value),
        }
    }
}
