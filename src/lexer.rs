use crate::models::{Position, Token, TokenType};

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
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
            '/' if chars.clone().nth(1) == Some('/') => {
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '\n' {
                        break;
                    }
                    chars.next();
                }
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

                Token::new(
                    if has_dot {
                        TokenType::FloatNumber
                    } else {
                        TokenType::Number
                    },
                    num,
                    Position {
                        line,
                        column: start_column,
                    },
                )
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
                    "int" => TokenType::IntType,
                    "float" => TokenType::FloatType,
                    "bool" => TokenType::BoolType,
                    "fetch" => TokenType::Fetch,
                    "push" => TokenType::Push,
                    "pop" => TokenType::Pop,
                    "loop" => TokenType::Loop,
                    "len" => TokenType::Len,
                    "break" => TokenType::Break,
                    _ => TokenType::Identifier,
                };
                Token::new(
                    token_type,
                    ident,
                    Position {
                        line,
                        column: start_column,
                    },
                )
            }
            '{' => {
                // Add this block
                let token = Token::new(
                    TokenType::LeftBrace,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            '}' => {
                let token = Token::new(
                    TokenType::RightBrace,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }
            ':' => {
                let start_column = column;
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::Assign,
                        ":=".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else if chars.peek() == Some(&'[') {
                    chars.next();
                    column += 1;
                    let mut type_str = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() {
                            type_str.push(chars.next().unwrap());
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    if chars.peek() == Some(&']') {
                        chars.next();
                        column += 1;
                        let token_type = match type_str.as_str() {
                            "int" => TokenType::ListIntType,
                            "float" => TokenType::ListFloatType,
                            "bool" => TokenType::ListBoolType,
                            _ => TokenType::Unknown,
                        };
                        Token::new(
                            token_type,
                            format!(":[{}]", type_str),
                            Position {
                                line,
                                column: start_column,
                            },
                        )
                    } else {
                        Token::new(
                            TokenType::Unknown,
                            format!(":[{}]", type_str),
                            Position {
                                line,
                                column: start_column,
                            },
                        )
                    }
                } else {
                    let mut type_str = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() {
                            type_str.push(chars.next().unwrap());
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    let token_type = match type_str.as_str() {
                        "int" => TokenType::IntType,
                        "float" => TokenType::FloatType,
                        "bool" => TokenType::BoolType,
                        _ => TokenType::Unknown,
                    };
                    Token::new(
                        token_type,
                        format!(":{}", type_str),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                }
            }

            '+' | '-' | '*' | '/' => {
                let token = Token::new(
                    TokenType::Operator,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            '(' => {
                let token = Token::new(
                    TokenType::LeftParen,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            ')' => {
                let token = Token::new(
                    TokenType::RightParen,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            ';' => {
                let token = Token::new(
                    TokenType::Termination,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            '>' => {
                let start_column = column;
                chars.next();
                column += 1;
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::GreaterThanOrEqual,
                        ">=".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::GreaterThan,
                        ">".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                };
                token
            }

            '<' => {
                let start_column = column;
                chars.next();
                column += 1;
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::LessThanOrEqual,
                        "<=".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::LessThan,
                        "<".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                };
                token
            }

            '=' => {
                let start_column = column;
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::Equal,
                        "==".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::Operator,
                        "=".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                }
            }

            '!' => {
                let start_column = column;
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::NotEqual,
                        "!=".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::Operator,
                        "!".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                }
            }

            '&' => {
                let start_column = column;
                chars.next();
                column += 1;
                if chars.peek() == Some(&'&') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::And,
                        "&&".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::Operator,
                        "&".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                }
            }

            '|' => {
                let start_column = column;
                chars.next();
                column += 1;
                if chars.peek() == Some(&'|') {
                    chars.next();
                    column += 1;
                    Token::new(
                        TokenType::Or,
                        "||".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                } else {
                    Token::new(
                        TokenType::Operator,
                        "|".to_string(),
                        Position {
                            line,
                            column: start_column,
                        },
                    )
                }
            }

            '[' => {
                let token = Token::new(
                    TokenType::LeftBracket,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            ']' => {
                let token = Token::new(
                    TokenType::RightBracket,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }

            ',' => {
                let token = Token::new(TokenType::Comma, ch.to_string(), Position { line, column });
                chars.next();
                column += 1;
                token
            }

            '.' => {
                let token = Token::new(TokenType::Dot, ch.to_string(), Position { line, column });
                chars.next();
                column += 1;
                token
            }

            _ => {
                let token = Token::new(
                    TokenType::Unknown,
                    ch.to_string(),
                    Position { line, column },
                );
                chars.next();
                column += 1;
                token
            }
        };
        tokens.push(token);
    }
    tokens.push(Token::new(
        TokenType::EOF,
        "".to_string(),
        Position { line, column },
    ));

    Ok(tokens)
}
