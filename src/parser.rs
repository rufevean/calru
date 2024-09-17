
use crate::models::{TokenType, Token, Position};
use crate::ast::{AST, ASTNode};
use crate::symbol_table::{SymbolTable, SymbolType, SymbolValue};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    pub current_token: Option<Token>,
    pub position: Position,
    pub symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_index: 0,
            current_token: None,
            position: Position { line: 1, column: 1 },
            symbol_table: SymbolTable::new(),
        };
        parser.advance();
        parser
    }

    pub fn parse_program(&mut self) -> Result<(Vec<AST>, SymbolTable), String> {
        let mut asts = Vec::new();

        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::EOF {
                break;
            }
            
            let ast = self.parse_statement()?;
            self.symbol_table.print(); 
            asts.push(ast);
        }

        Ok((asts, self.symbol_table.clone()))
    }

    pub fn parse_statement(&mut self) -> Result<AST, String> {
        if self.current_token_is(TokenType::Let) {
            self.parse_let_decl()
        } else if self.current_token_is(TokenType::Print) {
            self.parse_print()
        } else if self.current_token_is(TokenType::If) {
            self.parse_if_statement()
        } else {
            Err(format!(
                "Unexpected token {:?} at position {:?}. Expected 'let', 'stdout', or 'if'.",
                self.current_token, self.position
            ))
        }
    }

    // New method to parse if statements


pub fn parse_if_statement(&mut self) -> Result<AST, String> {
    if !self.current_token_is(TokenType::If) {
        return Err(format!("Expected 'if' at position {:?}. Found {:?}", self.position, self.current_token));
    }

    self.advance(); // Consume 'if'

    let condition = self.parse_expression()?;

    // Ensure that condition is a boolean expression or involves boolean comparison
    if !matches!(self.infer_type(&condition)?, SymbolType::Boolean) {
        return Err(format!("Expected boolean condition at position {:?}. Found {:?}", self.position, condition));
    }

    self.advance(); // Move to the next token after parsing the condition

    if !self.current_token_is(TokenType::Then) {
        return Err(format!("Expected 'then' after 'if' condition at position {:?}. Found {:?}", self.position, self.current_token));
    }

    self.advance(); // Consume 'then'

    let then_branch = self.parse_statement()?;

    let mut else_branch = None;
    if self.current_token_is(TokenType::Else) {
        self.advance(); // Consume 'else'
        else_branch = Some(self.parse_statement()?);
    }

    if !self.current_token_is(TokenType::End) {
        return Err(format!("Expected 'end' at position {:?}. Found {:?}", self.position, self.current_token));
    }

    self.advance(); // Consume 'end'

    Ok(AST::new(ASTNode::If {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: else_branch.map(Box::new),
    }))
}
    pub fn parse_let_decl(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Let) {
            return Err(format!("Expected 'let' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
        if !self.current_token_is(TokenType::Identifier) {
            return Err(format!("Expected identifier at position {:?}. Found {:?}", self.position, self.current_token));
        }

        let variable = self.current_token.as_ref().unwrap().value.clone();
        if self.symbol_table.lookup(&variable).is_some() {
            return Err(format!(
                "Variable '{}' already declared at position {:?}.",
                variable, self.position
            ));
        }

        self.advance(); 

        let symbol_type = if self.current_token_is(TokenType::IntType) {
            SymbolType::Int
        } else if self.current_token_is(TokenType::FloatType) {
            SymbolType::Float
        } else {
            return Err(format!(
                "Expected type 'int' or 'float' at position {:?}. Found {:?}.",
                self.position, self.current_token
            ));
        };
        self.advance(); 
        let expression = self.parse_assign_expr()?;

        let expr_type = self.infer_type(&expression)?;
        if expr_type != symbol_type {
            return Err(format!(
                "Type mismatch: cannot assign expression of type {:?} to variable of type {:?} at position {:?}.",
                expr_type, symbol_type, self.position
            ));
        }

        let value = self.evaluate_expression(&expression)?;

        self.symbol_table.insert(variable.clone(), symbol_type, value)
            .map_err(|e| format!("Error inserting symbol into symbol table: {}", e))?;

        Ok(AST::new(ASTNode::Assignment {
            variable,
            expression: Box::new(expression),
        }))
    }




pub fn parse_expression(&mut self) -> Result<AST, String> {
    let mut left = self.parse_term()?;
    let mut left_type = self.infer_type(&left)?;

    while self.current_token_is(TokenType::Operator)
        && (self.current_token.as_ref().unwrap().value == "+"
            || self.current_token.as_ref().unwrap().value == "-"
            || self.current_token.as_ref().unwrap().value == "=="
            || self.current_token.as_ref().unwrap().value == "!="
            || self.current_token.as_ref().unwrap().value == "&&"
            || self.current_token.as_ref().unwrap().value == "||")
    {
        let operator = self.current_token.as_ref().unwrap().value.clone();
        self.advance();

        let right = self.parse_term()?;
        let right_type = self.infer_type(&right)?;

        // Check for type compatibility based on the operator
        if (operator == "==" || operator == "!=") && (left_type == SymbolType::Int || left_type == SymbolType::Float) {
            left_type = SymbolType::Boolean;
        } else if (operator == "&&" || operator == "||") && left_type == SymbolType::Boolean {
            left_type = SymbolType::Boolean;
        } else if left_type != right_type {
            return Err(format!(
                "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                operator, left_type, right_type, self.position
            ));
        }

        left = AST::new(ASTNode::BinaryOperation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        });

        left_type = right_type;
    }

    Ok(left)
}
    pub fn parse_term(&mut self) -> Result<AST, String> {
        let mut left = self.parse_factor()?;
        let mut left_type = self.infer_type(&left)?;

        while self.current_token_is(TokenType::Operator)
            && (self.current_token.as_ref().unwrap().value == "*"
                || self.current_token.as_ref().unwrap().value == "/")
        {
            let operator = self.current_token.as_ref().unwrap().value.clone();
            self.advance();
            let right = self.parse_factor()?;
            let right_type = self.infer_type(&right)?;

            if left_type != right_type {
                return Err(format!(
                    "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                    operator, left_type, right_type, self.position
                ));
            }

            left = AST::new(ASTNode::BinaryOperation {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            });
            left_type = left_type;
        }

        Ok(left)
    }

 

pub fn parse_factor(&mut self) -> Result<AST, String> {
    match self.current_token {
        Some(ref token) if token.token_type == TokenType::Number => {
            let value = token.value.parse::<i64>()
                .map_err(|_| format!("Invalid integer format at position {:?}.", self.position))?;
            self.advance();
            Ok(AST::new(ASTNode::Int(value)))
        },
        Some(ref token) if token.token_type == TokenType::FloatNumber => {
            let value = token.value.parse::<f64>()
                .map_err(|_| format!("Invalid float format at position {:?}.", self.position))?;
            self.advance();
            Ok(AST::new(ASTNode::Float(value)))
        },
        Some(ref token) if token.token_type == TokenType::Boolean => {
            let value = token.value == "true";
            self.advance();
            Ok(AST::new(ASTNode::Boolean(value)))
        },
        Some(ref token) if token.token_type == TokenType::Identifier => {
            let value = token.value.clone();
            if self.symbol_table.lookup(&value).is_none() {
                return Err(format!("Undeclared variable '{}' at position {:?}.", value, self.position));
            }
            self.advance();
            Ok(AST::new(ASTNode::Identifier(value)))
        },
        _ => Err(format!("Unexpected token {:?} at position {:?}. Expected a number, float, identifier, or boolean.", self.current_token, self.position)),
    }
}
    pub fn parse_assign_expr(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Assign) {
            return Err(format!(
                "Expected ':=' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }

        self.advance();
        let expr = self.parse_expression()?;

        if !self.current_token_is(TokenType::Termination) {
            return Err(format!(
                "Expected ';' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }

        self.advance();
        Ok(expr)
    }

    pub fn parse_print(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Print) {
            return Err(format!("Expected 'stdout' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
        if !self.current_token_is(TokenType::LeftParen) || self.current_token.as_ref().unwrap().value != "(" {
            return Err(format!("Expected '(' after 'stdout' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
        let expression = self.parse_expression()?;

        if !self.current_token_is(TokenType::RightParen) || self.current_token.as_ref().unwrap().value != ")" {
            return Err(format!("Expected ')' after expression at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 

        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
Ok(AST::new(ASTNode::Print(Box::new(expression))))
    }

    // New method to handle boolean types
    pub fn infer_type(&self, ast: &AST) -> Result<SymbolType, String> {
        match ast.node {
            ASTNode::Int(_) => Ok(SymbolType::Int),
            ASTNode::Float(_) => Ok(SymbolType::Float),
            ASTNode::Boolean(_) => Ok(SymbolType::Boolean), // Added boolean type
            ASTNode::Identifier(ref name) => {
                self.symbol_table.lookup(name)
                    .map(|symbol| symbol.symbol_type.clone())
                    .ok_or_else(|| format!("Undefined variable '{}' at position {:?}", name, self.position))
            },
            ASTNode::BinaryOperation { ref left, ref right, .. } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                if left_type != right_type {
                    Err(format!("Type mismatch in binary operation. Left: {:?}, Right: {:?}", left_type, right_type))
                } else {
                    Ok(left_type)
                }
            },
            _ => Err(format!("Cannot infer type for AST node {:?}", ast.node)),
        }
    }

    // Add this method to evaluate expressions if needed
    pub fn evaluate_expression(&self, ast: &AST) -> Result<SymbolValue, String> {
        match ast.node {
            ASTNode::Int(value) => Ok(SymbolValue::Int(value)),
            ASTNode::Float(value) => Ok(SymbolValue::Float(value)),
            ASTNode::Boolean(value) => Ok(SymbolValue::Boolean(value)), // Added boolean evaluation
            ASTNode::Identifier(ref name) => {
                self.symbol_table.lookup(name)
                    .map(|symbol| symbol.value.clone())
                    .ok_or_else(|| format!("Undefined variable '{}' at position {:?}", name, self.position))
            },
            ASTNode::BinaryOperation { ref left, ref right, ref operator } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;

                match operator.as_str() {
                    "+" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l + r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l + r)),
                        _ => Err(format!("Type mismatch for '+' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "-" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l - r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l - r)),
                        _ => Err(format!("Type mismatch for '-' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "*" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l * r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l * r)),
                        _ => Err(format!("Type mismatch for '*' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "/" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l / r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l / r)),
                        _ => Err(format!("Type mismatch for '/' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    _ => Err(format!("Unknown operator '{}' in binary operation.", operator)),
                }
            },
            _ => Err(format!("Cannot evaluate AST node {:?}", ast.node)),
        }
    }

    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
            self.current_index += 1;
        } else {
            self.current_token = None;
        }
    }

    pub fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map(|token| token.token_type == token_type)
            .unwrap_or(false)
    }
}
