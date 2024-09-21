
use crate::symbol_table::{SymbolTable, SymbolType, SymbolValue};
use crate::ast::{AST, ASTNode};
use std::fmt;

pub struct Interpreter {
    symbol_table: SymbolTable,
}

impl Interpreter {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Interpreter { symbol_table }
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
            ASTNode::Boolean(b) => {
                // Handle Boolean nodes if needed
                println!("Boolean value: {}", b);
            },
            ASTNode::If { condition, then_branch, else_branch } => {
                // Handle If statements (needs further implementation for actual branching logic)
                let condition_value = self.evaluate_expression(condition)?;
                // For simplicity, assuming `condition_value` is a boolean here.
                if let SymbolValue::Boolean(true) = condition_value {
                    self.execute_statement(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute_statement(else_branch)?;
                }
            },
            _ => return Err(format!("Unsupported statement {:?}", ast.node)),
        }
        Ok(())
    }

    fn evaluate_expression(&self, expression: &AST) -> Result<SymbolValue, String> {
        match &expression.node {
            ASTNode::Int(value) => Ok(SymbolValue::Int(*value)),
            ASTNode::Float(value) => Ok(SymbolValue::Float(*value)),
            ASTNode::Boolean(value) => Ok(SymbolValue::Boolean(*value)),
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
                            "+" => SymbolValue::Int(left_val + right_val),
                            "-" => SymbolValue::Int(left_val - right_val),
                            "*" => SymbolValue::Int(left_val * right_val),
                            "/" => SymbolValue::Int(left_val / right_val),
                            ">" => SymbolValue::Boolean(left_val > right_val),
                            "<" => SymbolValue::Boolean(left_val < right_val),
                            ">=" => SymbolValue::Boolean(left_val >= right_val),
                            "<=" => SymbolValue::Boolean(left_val <= right_val),
                            "==" => SymbolValue::Boolean(left_val == right_val),
                            "!=" => SymbolValue::Boolean(left_val != right_val),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(result)
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => SymbolValue::Float(left_val + right_val),
                            "-" => SymbolValue::Float(left_val - right_val),
                            "*" => SymbolValue::Float(left_val * right_val),
                            "/" => SymbolValue::Float(left_val / right_val),
                            ">" => SymbolValue::Boolean(left_val > right_val),
                            "<" => SymbolValue::Boolean(left_val < right_val),
                            ">=" => SymbolValue::Boolean(left_val >= right_val),
                            "<=" => SymbolValue::Boolean(left_val <= right_val),
                            "==" => SymbolValue::Boolean(left_val == right_val),
                            "!=" => SymbolValue::Boolean(left_val != right_val),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(result)
                    },
                    (SymbolValue::Int(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => SymbolValue::Float((left_val as f64) + right_val),
                            "-" => SymbolValue::Float((left_val as f64) - right_val),
                            "*" => SymbolValue::Float((left_val as f64) * right_val),
                            "/" => SymbolValue::Float((left_val as f64) / right_val),
                            ">" => SymbolValue::Boolean((left_val as f64) > right_val),
                            "<" => SymbolValue::Boolean((left_val as f64) < right_val),
                            ">=" => SymbolValue::Boolean((left_val as f64) >= right_val),
                            "<=" => SymbolValue::Boolean((left_val as f64) <= right_val),
                            "==" => SymbolValue::Boolean((left_val as f64) == right_val),
                            "!=" => SymbolValue::Boolean((left_val as f64) != right_val),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(result)
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Int(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => SymbolValue::Float(left_val + (right_val as f64)),
                            "-" => SymbolValue::Float(left_val - (right_val as f64)),
                            "*" => SymbolValue::Float(left_val * (right_val as f64)),
                            "/" => SymbolValue::Float(left_val / (right_val as f64)),
                            ">" => SymbolValue::Boolean(left_val > (right_val as f64)),
                            "<" => SymbolValue::Boolean(left_val < (right_val as f64)),
                            ">=" => SymbolValue::Boolean(left_val >= (right_val as f64)),
                            "<=" => SymbolValue::Boolean(left_val <= (right_val as f64)),
                            "==" => SymbolValue::Boolean(left_val == (right_val as f64)),
                            "!=" => SymbolValue::Boolean(left_val != (right_val as f64)),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(result)
                    },
                    (SymbolValue::Boolean(left_val), SymbolValue::Boolean(right_val)) => {
                        let result = match operator.as_str() {
                            "&&" => SymbolValue::Boolean(left_val && right_val),
                            "||" => SymbolValue::Boolean(left_val || right_val),
                            "==" => SymbolValue::Boolean(left_val == right_val),
                            "!=" => SymbolValue::Boolean(left_val != right_val),
                            _ => return Err(format!("Unsupported operator '{}' in binary operation.", operator)),
                        };
                        Ok(result)
                    },
                    _ => Err("Type mismatch in binary operation.".to_string()),
                }
            },
            _ => Err(format!("Cannot evaluate expression node {:?}", expression.node)),
        }
    }
    fn infer_type(&self, node: &AST) -> Result<SymbolType, String> {
        match &node.node {
            ASTNode::Int(_) => Ok(SymbolType::Int),
            ASTNode::Float(_) => Ok(SymbolType::Float),
            ASTNode::Boolean(_) => Ok(SymbolType::Boolean),
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
            ASTNode::If { condition, then_branch, else_branch } => {
                let condition_type = self.infer_type(condition)?;
                let then_type = self.infer_type(then_branch)?;
                let else_type = if let Some(else_branch) = else_branch {
                    self.infer_type(else_branch)?
                } else {
                    then_type.clone() // Default to the `then_branch` type if `else_branch` is None
                };

                if condition_type != SymbolType::Boolean {
                    Err(format!("Condition in 'If' statement must be of type Boolean."))
                } else if then_type == else_type {
                    Ok(then_type)
                } else {
                    Err(format!(
                        "Type mismatch in 'If' statement branches. Then branch type: {:?}, Else branch type: {:?}.",
                        then_type, else_type
                    ))
                }
            }
        }
    }
}

impl fmt::Display for SymbolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolValue::Int(value) => write!(f, "{}", value),
            SymbolValue::Float(value) => write!(f, "{}", value),
            SymbolValue::Boolean(value) => write!(f, "{}", value), // Add this line
        }
    }
}
