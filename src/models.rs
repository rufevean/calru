#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Identifier,
    Number,
    FloatNumber,
    Boolean, // Add boolean token type
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
    BoolType, // Add boolean type
    // If statement
    If,
    Then,
    Else,
    End,
    // Comparison operators
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    // Logical operators
    And,
    Or,
    // List tokens
    LeftBracket,  // Add left bracket for list
    RightBracket, // Add right bracket for list
    Comma,        // Add comma for list elements
    Colon, 
    ListIntType,   // Add this line
    ListFloatType, // Add this line
    ListBoolType
}

#[derive(Debug, Clone, Copy, PartialEq)]
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