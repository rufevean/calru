
use crate::models::{TokenType, Token, Position};

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    pub current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_index: 0,
            current_token: None,
        };
        parser.advance(); // Initialize the first token
        parser
    }

    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
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
}
