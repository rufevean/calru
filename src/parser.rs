
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

    pub fn parse_program(&mut self) -> Result<Vec<AST>, String> {
        let mut asts = Vec::new();

        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::EOF {
                break;
            }
            
            let ast = self.parse_statement()?;
            self.symbol_table.print(); // Print symbol table after each statement
            asts.push(ast);
        }

        Ok(asts)
    }

    pub fn parse_statement(&mut self) -> Result<AST, String> {
        if self.current_token_is(TokenType::Let) {
            self.parse_let_decl()
        } else if self.current_token_is(TokenType::Print) {
            self.parse_print()
        } else {
            Err(format!(
                "Unexpected token {:?} at position {:?}. Expected 'let' or 'stdout'.",
                self.current_token, self.position
            ))
        }
    }

    pub fn parse_let_decl(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Let) {
            return Err(format!("Expected 'let' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Move past 'let'

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

        self.advance(); // Move past identifier

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
        self.advance(); // Move past type

        let expression = self.parse_assign_expr()?;

        let expr_type = self.infer_type(&expression)?;
        if expr_type != symbol_type {
            return Err(format!(
                "Type mismatch: cannot assign expression of type {:?} to variable of type {:?} at position {:?}.",
                expr_type, symbol_type, self.position
            ));
        }

        let value = self.evaluate_expression(&expression)?;

        // Store the variable and its value in the symbol table
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
                || self.current_token.as_ref().unwrap().value == "-")
        {
            let operator = self.current_token.as_ref().unwrap().value.clone();
            self.advance();

            let right = self.parse_term()?;
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
            Some(ref token) if token.token_type == TokenType::Identifier => {
                let value = token.value.clone();
                if self.symbol_table.lookup(&value).is_none() {
                    return Err(format!("Undeclared variable '{}' at position {:?}.", value, self.position));
                }
                self.advance();
                Ok(AST::new(ASTNode::Identifier(value)))
            },
            _ => Err(format!("Unexpected token {:?} at position {:?}. Expected a number, float, or identifier.", self.current_token, self.position)),
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

        self.advance(); // Move past 'stdout'

        if !self.current_token_is(TokenType::LeftParen) || self.current_token.as_ref().unwrap().value != "(" {
            return Err(format!("Expected '(' after 'stdout' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Move past '('

        let expression = self.parse_expression()?;

        if !self.current_token_is(TokenType::RightParen) || self.current_token.as_ref().unwrap().value != ")" {
            return Err(format!("Expected ')' after expression at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Move past ')'

        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Move past ';'

        Ok(AST::new(ASTNode::Print(Box::new(expression))))
    }

    /* UTIL METHODS */

    pub fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.as_ref().map_or(false, |token| token.token_type == token_type)
    }

    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
            self.update_position();
            self.current_index += 1;
        } else {
            self.current_token = None;
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.current_index < self.tokens.len() {
            Some(&self.tokens[self.current_index])
        } else {
            None
        }
    }

    fn update_position(&mut self) {
        if let Some(token) = &self.current_token {
            self.position = token.position.clone();
        }
    }

    fn infer_type(&self, node: &AST) -> Result<SymbolType, String> {
        match &node.node {
            ASTNode::Int(_) => Ok(SymbolType::Int),
            ASTNode::Float(_) => Ok(SymbolType::Float),
            ASTNode::Identifier(ref id) => {
                self.symbol_table.lookup(id)
                    .map(|symbol| symbol.symbol_type.clone())
                    .ok_or_else(|| format!("Undeclared variable '{}' at position {:?}.", id, self.position))
            },
            ASTNode::BinaryOperation { left, right, .. } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                if left_type == right_type {
                    Ok(left_type)
                } else {
                    Err(format!(
                        "Type mismatch in binary operation at position {:?}. Expected {:?}, found {:?}.",
                        self.position, left_type, right_type
                    ))
                }
            },
            ASTNode::Assignment { expression, .. } => self.infer_type(expression),
            ASTNode::Print(expression) => self.infer_type(expression),
        }
    }

    fn evaluate_expression(&self, expression: &AST) -> Result<SymbolValue, String> {
        match &expression.node {
            ASTNode::Int(value) => Ok(SymbolValue::Int(*value)),
            ASTNode::Float(value) => Ok(SymbolValue::Float(*value)),
            ASTNode::Identifier(id) => {
                self.symbol_table.lookup(id)
                    .map(|symbol| symbol.value.clone())
                    .ok_or_else(|| format!("Undeclared variable '{}' at position {:?}.", id, self.position))
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
                            _ => return Err(format!("Unsupported operator '{}' at position {:?}", operator, self.position)),
                        };
                        Ok(SymbolValue::Int(result))
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => left_val + right_val,
                            "-" => left_val - right_val,
                            "*" => left_val * right_val,
                            "/" => left_val / right_val,
                            _ => return Err(format!("Unsupported operator '{}' at position {:?}", operator, self.position)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                    (SymbolValue::Int(left_val), SymbolValue::Float(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => (left_val as f64) + right_val,
                            "-" => (left_val as f64) - right_val,
                            "*" => (left_val as f64) * right_val,
                            "/" => (left_val as f64) / right_val,
                            _ => return Err(format!("Unsupported operator '{}' at position {:?}", operator, self.position)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                    (SymbolValue::Float(left_val), SymbolValue::Int(right_val)) => {
                        let result = match operator.as_str() {
                            "+" => left_val + (right_val as f64),
                            "-" => left_val - (right_val as f64),
                            "*" => left_val * (right_val as f64),
                            "/" => left_val / (right_val as f64),
                            _ => return Err(format!("Unsupported operator '{}' at position {:?}", operator, self.position)),
                        };
                        Ok(SymbolValue::Float(result))
                    },
                }
            },
            _ => Err(format!("Cannot evaluate expression node {:?}", expression.node)),
        }
    }
}
