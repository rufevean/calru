#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
}

#[derive(Debug,PartialEq, Clone, Copy)]
pub enum TokenType {
    Identifier,
    Number,
    FloatNumber,
    Let,
    Operator,
    Assign,
    Unknown,
    Termination,
    EOF,
    Print,
    RightParen,
    LeftParen,
    // Data types
    IntType,
    FloatType,
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, position: Position) -> Self {
        Token {
            token_type,
            value,
            position,
        }
    }
}
