
use crate::models::{TokenType, Token, Position};
use crate::ast::{AST, ASTNode};
use crate::symbol_table::{SymbolTable, SymbolType};

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

    pub fn parse_statement(&mut self, symbol_table: &mut SymbolTable) -> Result<AST, String> {
        if self.current_token_is(TokenType::Let) {
            self.parse_let_decl(symbol_table)
        } else {
            Err(format!(
                "Unexpected token {:?} at position {:?}. Expected 'let' to start a declaration.",
                self.current_token, self.position
            ))
        }
    }

    pub fn parse_let_decl(&mut self, symbol_table: &mut SymbolTable) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Let) {
            return Err(format!("Expected 'let' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance();

        if !self.current_token_is(TokenType::Identifier) {
            return Err(format!("Expected identifier at position {:?}. Found {:?}", self.position, self.current_token));
        }

        let variable = self.current_token.as_ref().unwrap().value.clone();
        if symbol_table.lookup(&variable).is_some() {
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

        symbol_table.insert(variable.clone(), symbol_type.clone())
            .map_err(|e| format!("Error inserting symbol into symbol table: {}", e))?;

        let expression = self.parse_assign_expr()?;

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
                let value = token.value.parse::<f64>()
                    .map_err(|_| format!("Invalid number format at position {:?}.", self.position))?;
                self.advance();
                Ok(AST::new(ASTNode::Number(value)))
            },
            Some(ref token) if token.token_type == TokenType::FloatNumber => {
                let value = token.value.parse::<f64>()
                    .map_err(|_| format!("Invalid float format at position {:?}.", self.position))?;
                self.advance();
                Ok(AST::new(ASTNode::Number(value)))
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
            return Err(format!("Expected ':=' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance();
        let expr = self.parse_expression()?;

        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance();
        Ok(expr)
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
            ASTNode::Number(_) => Ok(SymbolType::Float),
            ASTNode::Identifier(ref id) => {
                self.symbol_table.lookup(id)
                    .cloned()
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
        }
    }
}
