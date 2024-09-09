
use calru::lexer::lexer;
use calru::models::{TokenType};

#[test]
fn test_lexer() {
    let input = "let var1 :int := 3.14; // this is a comment";
    let tokens = lexer(input);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::IntType);
    assert_eq!(tokens[3].token_type, TokenType::Assign);
    assert_eq!(tokens[4].token_type, TokenType::FloatNumber);
    assert_eq!(tokens[5].token_type, TokenType::Termination);
}

#[test]
fn test_lexer_with_float() {
    let input = "let x := 3.14;";
    let tokens = lexer(input);

    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::FloatNumber);
    assert_eq!(tokens[4].token_type, TokenType::Termination);
    assert_eq!(tokens[5].token_type, TokenType::EOF);
}

#[test]
fn test_lexer_with_comments() {
    let input = "let x := 5; // comment\nlet y := 10;";
    let tokens = lexer(input);

    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number);
    assert_eq!(tokens[4].token_type, TokenType::Termination);
    assert_eq!(tokens[5].token_type, TokenType::Let);
    assert_eq!(tokens[6].token_type, TokenType::Identifier);
    assert_eq!(tokens[7].token_type, TokenType::Assign);
    assert_eq!(tokens[8].token_type, TokenType::Number);
    assert_eq!(tokens[9].token_type, TokenType::Termination);
    assert_eq!(tokens[10].token_type, TokenType::EOF);
}


