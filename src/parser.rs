
use crate::models::{TokenType, Token, Position};
use crate::ast::{AST, ASTNode};

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    pub current_token: Option<Token>,
    pub position : Position, 
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_index: 0,
            current_token: None,
            position : Position {line: 1, column : 1}
        };
        parser.advance(); 
        parser
    }

    pub fn parse_statement(&mut self) -> Result<AST, String> {
        if self.current_token_is(TokenType::Let){
            self.parse_let_decl()
        }else{
            Err(
                format!(
                    "Unexpected token {:?} at position {:?}",
                    self.current_token,self.position
                )
            )
        }
    }


pub fn parse_let_decl(&mut self) -> Result<AST, String> {
    if !self.current_token_is(TokenType::Let) {
        return Err(format!("Expected 'let' at position {:?}", self.position));
    }

    self.advance();

    if !self.current_token_is(TokenType::Identifier) {
        return Err(format!("Expected identifier at position {:?}", self.position));
    }

    let variable = self.current_token.as_ref().unwrap().value.clone();
    self.advance();

    if !self.current_token_is(TokenType::IntType) && !self.current_token_is(TokenType::FloatType) {
        return Err(format!("Expected type (int/float) at position {:?}", self.position));
    }
    self.advance();

    let expression = self.parse_assign_expr()?;

    Ok(AST::new(ASTNode::Assignment {
        variable,
        expression: Box::new(expression),
    }))
}

pub fn parse_expression(&mut self) -> Result<AST, String> {
    let mut left = self.parse_term()?; 

    while self.current_token_is(TokenType::Operator) &&
          (self.current_token.as_ref().unwrap().value == "+" ||
           self.current_token.as_ref().unwrap().value == "-") {

        let operator = self.current_token.as_ref().unwrap().value.clone();
        self.advance();
        let right = self.parse_term()?;

        left = AST::new(ASTNode::BinaryOperation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        });
    }

    Ok(left) 
}




 
pub fn parse_term(&mut self) -> Result<AST, String> {
    let mut left = self.parse_factor()?;

    while self.current_token_is(TokenType::Operator) &&
          (self.current_token.as_ref().unwrap().value == "*" ||
           self.current_token.as_ref().unwrap().value == "/") {

        let operator = self.current_token.as_ref().unwrap().value.clone();
        self.advance();
        let right = self.parse_factor()?; 

        left = AST::new(ASTNode::BinaryOperation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        });
    }

    Ok(left) 
}



pub fn parse_factor(&mut self) -> Result<AST, String> {
    match self.current_token {
        Some(ref token) if token.token_type == TokenType::Number => {
            let value = token.value.parse::<f64>()
                .map_err(|_| format!("Invalid number format at position {:?}", self.position))?;
            self.advance();
            Ok(AST::new(ASTNode::Number(value)))
        },
        Some(ref token) if token.token_type == TokenType::FloatNumber => {
            let value = token.value.parse::<f64>()
                .map_err(|_| format!("Invalid float format at position {:?}", self.position))?;
            self.advance();
            Ok(AST::new(ASTNode::Number(value)))
        },
        Some(ref token) if token.token_type == TokenType::Identifier => {
            let value = token.value.clone();
            self.advance();
            Ok(AST::new(ASTNode::Identifier(value)))
        },
        _ => Err(format!("Unexpected token {:?} at position {:?}", self.current_token, self.position)),
    }
}




pub fn parse_assign_expr(&mut self) -> Result<AST, String> {
    if !self.current_token_is(TokenType::Assign) {
        return Err(format!("Expected ':=' at position {:?}", self.position));
    }

    self.advance();
    let expr = self.parse_expression()?;

    if !self.current_token_is(TokenType::Termination) {
        return Err(format!("Expected ';' at position {:?}", self.position));
    }

    self.advance();
    Ok(expr)
}

    
            /* UTIL METHODS */

    pub fn current_token_is(&self, token_type : TokenType) -> bool{
        self.current_token.as_ref().map_or(false,|token| token.token_type ==   token_type)
    }
    
    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
            self.update_position() ;
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

    fn update_position(&mut self){
        if let Some(token) = &self.current_token{
            self.position = token.position.clone(); 
        }
    }
}
