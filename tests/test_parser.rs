
#[cfg(test)]
mod tests {
    use calru::models::{Token, TokenType, Position};
    use calru::parser::Parser;

    #[test]
    fn test_parser_initialization() {
        let tokens = vec![
            Token {
                token_type: TokenType::Let,
                value: "let".to_string(),
                position: Position { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::Identifier,
                value: "var".to_string(),
                position: Position { line: 1, column: 5 },
            },
        ];
        let mut parser = Parser::new(tokens);

        assert_eq!(parser.current_token.as_ref().unwrap().token_type, TokenType::Let);
        parser.advance();
        assert_eq!(parser.current_token.as_ref().unwrap().token_type, TokenType::Identifier);
    }
}
