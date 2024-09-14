
use crate::models::{Position, Token, TokenType};

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        let token = match ch {
            ' ' | '\t' => {
                chars.next();
                column += 1;
                continue;
            }
            '\n' => {
                chars.next();
                line += 1;
                column = 1;
                continue;
            }

            '0'..='9' => {
                let start_column = column;
                let mut num = String::new();
                let mut has_dot = false;

                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() {
                        num.push(chars.next().unwrap());
                        column += 1;
                    } else if digit == '.' {
                        if has_dot {
                            break;
                        }
                        has_dot = true;
                        num.push(chars.next().unwrap());
                        column += 1;
                    } else {
                        break;
                    }
                }

                Token {
                    token_type: if has_dot {
                        TokenType::FloatNumber
                    } else {
                        TokenType::Number
                    },
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
                let token_type = match ident.as_str() {
                    "let" => TokenType::Let,
                    "stdout" => TokenType::Print,
                    "if" => TokenType::If,
                    "then" => TokenType::Then,
                    "else" => TokenType::Else,
                    "end" => TokenType::End,
                    "true" | "false" => TokenType::Boolean, // Handle boolean literals
                    _ => TokenType::Identifier,
                };
                Token {
                    token_type,
                    value: ident,
                    position: Position { line, column: start_column },
                }
            }

            ':' => {
                let mut token;
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next(); 
                    token = Token {
                        token_type: TokenType::Assign,
                        value: ":=".to_string(),
                        position: Position { line, column },
                    };
                    column += 2;
                } else {
                    let next_word: String = chars.clone().take(5).collect();
                    if next_word.starts_with("int") {
                        chars.by_ref().take(3).for_each(|_| column += 1);
                        token = Token {
                            token_type: TokenType::IntType,
                            value: "int".to_string(),
                            position: Position { line, column },
                        };
                    } else if next_word.starts_with("float") {
                        chars.by_ref().take(5).for_each(|_| column += 1);
                        token = Token {
                            token_type: TokenType::FloatType,
                            value: "float".to_string(),
                            position: Position { line, column },
                        };
                    } else {
                        token = Token {
                            token_type: TokenType::Operator,
                            value: ":".to_string(),
                            position: Position { line, column },
                        };
                        column += 1;
                    }
                }
                token
            }

            '/' => {
                if chars.peek() == Some(&'/') {
                    chars.next(); // Consume the '/'
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                    column = if let Some(&'\n') = chars.peek() { column + 1 } else { column };
                    continue;
                } else {
                    Token {
                        token_type: TokenType::Operator,
                        value: ch.to_string(),
                        position: Position { line, column },
                    }
                }
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

            '(' => {
                let token = Token {
                    token_type: TokenType::LeftParen,
                    value: ch.to_string(),
                    position: Position { line, column },
                };
                chars.next();
                column += 1;
                token
            }

            ')' => {
                let token = Token {
                    token_type: TokenType::RightParen,
                    value: ch.to_string(),
                    position: Position { line, column },
                };
                chars.next();
                column += 1;
                token
            }

            ';' => {
                let token = Token {
                    token_type: TokenType::Termination,
                    value: ch.to_string(),
                    position: Position { line, column },
                };
                chars.next();
                column += 1;
                token
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
    tokens.push(Token {
        token_type: TokenType::EOF,
        value: "".to_string(),
        position: Position { line, column },
    });

    tokens
}
