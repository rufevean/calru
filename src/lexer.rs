use crate::models::Token; 
use crate::models::Position;
use crate::models::TokenType;
pub fn lexer (input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        let token = match ch {
            '0'..='9' => {
                let start_column = column;
                let mut num = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() {
                        num.push(chars.next().unwrap());
                        column += 1;
                    } else {
                        break;
                    }
                }
                Token {
                    token_type: TokenType::Number,
                    value: num,
                    position: Position { line, column: start_column },
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start_column = column;
                let mut ident = String::new();
                while let Some(&letter) = chars.peek() {
                    if letter.is_alphanumeric() || letter == '_' {
                        ident.push(chars.next().unwrap());
                        column += 1;
                    } else {
                        break;
                    }
                }
                let token_type = if ident == "let" {
                    TokenType::Let                } else {
                    TokenType::Identifier
                };
                Token {
                    token_type,
                    value: ident,
                    position: Position { line, column: start_column },
                }
            }
            ':' => {
                let token;
                chars.next();
                if chars.peek().map_or(false, |&next_ch| next_ch == '=') {
                    chars.next();  
                        token = Token {
                        token_type: TokenType::Assign,
                        value: ":=".to_string(),
                        position: Position { line, column },
                    };
                    column += 2; 
                } else {
                    token = Token {
                        token_type: TokenType::Operator,
                        value: ch.to_string(),
                        position: Position { line, column },
                    };
                    column += 1; 
                }

                chars.next(); 
                token
            }
            '+' | '-' | '*' | '/' => {
                let token = Token {
                    token_type: TokenType::Operator,
                    value: ch.to_string(),
                    position: Position { line, column },
                };
                chars.next();
                column += 1;
                token
            }
            ' ' | '\t' => {
                chars.next();
                column += 1;
                continue;             }
            '\n' => {
                chars.next();
                line += 1;
                column = 1;
                continue; 
            }
                        _ => {
                let token = Token {
                    token_type: TokenType::Unknown,
                    value: ch.to_string(),
                    position: Position { line, column },
                };
                chars.next();
                column += 1;
                token
            }
        };
        tokens.push(token);
    }

    tokens
}
