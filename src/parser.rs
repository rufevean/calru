
use crate::models::{TokenType, Token, Position};

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
        parser.advance(); // Initialize the first token
        parser
    }

    pub fn parse_statement(&mut self) -> Result<(), String> {
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

    pub fn parse_let_decl(&mut self) -> Result<(),String>{
        if !self.current_token_is(TokenType::Let){
            return Err(format!("Expected 'let' at position {:?}", self.position));
        }

        self.advance();

        if !self.current_token_is(TokenType::Identifier){
            return Err(format!("Expected identifier at position {:?}",self.position));
        }

        self.advance();

                // Expect Type (int or float)
        if !self.current_token_is(TokenType::IntType) && !self.current_token_is(TokenType::FloatType) {
            return Err(format!("Expected type (int/float) at position {:?}", self.position));
        }
        self.advance();


        self.parse_assign_expr()?;


        Ok(())

        
    }

    pub fn parse_expression(&mut self) -> Result<(),String>{

        self.parse_term()?;

        while self.current_token_is(TokenType::Operator) &&
            (self.current_token.as_ref().unwrap().value == "+" ||
            self.current_token.as_ref().unwrap().value=="-"){
        self.advance();
        self.parse_term()?;
    }
        Ok(())
    }


    pub fn parse_term(&mut self) -> Result<(),String>{

        self.parse_factor()?;

        while self.current_token_is(TokenType::Operator) &&
        ( self.current_token.as_ref().unwrap().value == "*" ||
        self.current_token.as_ref().unwrap().value == "/"){
    self.advance();
    self.parse_factor();
}
Ok(())
    }

    pub fn parse_factor(&mut self) -> Result<(),String>{
        match self.current_token{
            Some(ref token) if token.token_type == TokenType::Number || token.token_type == TokenType::Identifier => {
                self.advance();
                Ok(())
            },
            _ => Err(format!("Unexpected token {:?} at position {:?}",self.current_token,self.position))
                    }
    }

    pub fn parse_assign_expr(&mut self)-> Result<(),String>{
        if self.current_token_is(TokenType::Assign){
            self.advance();
            self.parse_expression()?;
            if self.current_token_is(TokenType::Termination){
                self.advance();
                Ok(())
            }else{
                Err(format!("Expected ';' at position {:?}",self.position))
            }
            
        }else{
            Err(format!("Expected  ':=' at position {:?}",self.position))
        }
    }
    
            /* UTIL METHODS */

    pub fn current_token_is(&self, token_type : TokenType) -> bool{
        self.current_token.as_ref().map_or(false,|token| token.token_type ==   token_type)
    }
    
    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
            self.update_position() ;
            self.current_index += 1; // Move to the next token
        } else {
            self.current_token = None; // End of token list
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
