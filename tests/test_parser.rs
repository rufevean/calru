
use calru::parser::Parser;
use calru::models::{TokenType, Token};
use calru::ast::{AST, ASTNode};
use calru::models::Position;
use calru::symbol_table::SymbolTable; // Import SymbolTable for the tests

#[test]
fn test_let_decl() {
    let tokens = vec![
        Token { token_type: TokenType::Let, value: "let".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::Identifier, value: "x".to_string(), position: Position { line: 1, column: 5 } },
        Token { token_type: TokenType::IntType, value: "int".to_string(), position: Position { line: 1, column: 7 } },
        Token { token_type: TokenType::Assign, value: ":=".to_string(), position: Position { line: 1, column: 11 } },
        Token { token_type: TokenType::Number, value: "42".to_string(), position: Position { line: 1, column: 14 } },
        Token { token_type: TokenType::Termination, value: ";".to_string(), position: Position { line: 1, column: 16 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 17 } },
    ];

    let mut symbol_table = SymbolTable::new(); // Initialize the symbol table
    let mut parser = Parser::new(tokens);

    let ast = parser.parse_statement(&mut symbol_table).expect("Failed to parse statement");

    let expected_ast = AST::new(ASTNode::Assignment {
        variable: "x".to_string(),
        expression: Box::new(AST::new(ASTNode::Int(42))),
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_expression_parsing() {
    let tokens = vec![
        Token { token_type: TokenType::Number, value: "3".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::Operator, value: "+".to_string(), position: Position { line: 1, column: 2 } },
        Token { token_type: TokenType::Number, value: "4".to_string(), position: Position { line: 1, column: 4 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 5 } },
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression().expect("Failed to parse expression");

    let expected_ast = AST::new(ASTNode::BinaryOperation {
        operator: "+".to_string(),
        left: Box::new(AST::new(ASTNode::Int(3))),
        right: Box::new(AST::new(ASTNode::Int(4))),
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_expression_with_precedence() {
    let tokens = vec![
        Token { token_type: TokenType::Number, value: "2".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::Operator, value: "*".to_string(), position: Position { line: 1, column: 2 } },
        Token { token_type: TokenType::Number, value: "3".to_string(), position: Position { line: 1, column: 4 } },
        Token { token_type: TokenType::Operator, value: "+".to_string(), position: Position { line: 1, column: 5 } },
        Token { token_type: TokenType::Number, value: "4".to_string(), position: Position { line: 1, column: 6 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 7 } },
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression().expect("Failed to parse expression");

    let expected_ast = AST::new(ASTNode::BinaryOperation {
        operator: "+".to_string(),
        left: Box::new(AST::new(ASTNode::BinaryOperation {
            operator: "*".to_string(),
            left: Box::new(AST::new(ASTNode::Int(2))),
            right: Box::new(AST::new(ASTNode::Int(3))),
        })),
        right: Box::new(AST::new(ASTNode::Int(4))),
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_complex_expression() {
    let tokens = vec![
        Token { token_type: TokenType::Number, value: "2".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::Operator, value: "*".to_string(), position: Position { line: 1, column: 2 } },
        Token { token_type: TokenType::Number, value: "3".to_string(), position: Position { line: 1, column: 4 } },
        Token { token_type: TokenType::Operator, value: "+".to_string(), position: Position { line: 1, column: 5 } },
        Token { token_type: TokenType::Number, value: "4".to_string(), position: Position { line: 1, column: 6 } },
        Token { token_type: TokenType::Operator, value: "*".to_string(), position: Position { line: 1, column: 7 } },
        Token { token_type: TokenType::Number, value: "5".to_string(), position: Position { line: 1, column: 8 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 9 } },
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression().expect("Failed to parse expression");

    let expected_ast = AST::new(ASTNode::BinaryOperation {
        operator: "+".to_string(),
        left: Box::new(AST::new(ASTNode::BinaryOperation {
            operator: "*".to_string(),
            left: Box::new(AST::new(ASTNode::Int(2))),
            right: Box::new(AST::new(ASTNode::Int(3))),
        })),
        right: Box::new(AST::new(ASTNode::BinaryOperation {
            operator: "*".to_string(),
            left: Box::new(AST::new(ASTNode::Int(4))),
            right: Box::new(AST::new(ASTNode::Int(5))),
        })),
    });

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_missing_variable() {
    let tokens = vec![
        Token { token_type: TokenType::Let, value: "let".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::IntType, value: "int".to_string(), position: Position { line: 1, column: 5 } },
        Token { token_type: TokenType::Assign, value: ":=".to_string(), position: Position { line: 1, column: 8 } },
        Token { token_type: TokenType::Number, value: "42".to_string(), position: Position { line: 1, column: 11 } },
        Token { token_type: TokenType::Termination, value: ";".to_string(), position: Position { line: 1, column: 13 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 14 } },
    ];

    let mut parser = Parser::new(tokens);
    let mut symbol_table = SymbolTable::new(); // Initialize symbol table

    let result = parser.parse_statement(&mut symbol_table);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Expected identifier at position Position { line: 1, column: 5 }. Found Some(Token { token_type: IntType, value: \"int\", position: Position { line: 1, column: 5 } })"
    );
}

#[test]
fn test_unexpected_token() {
    let tokens = vec![
        Token { token_type: TokenType::Number, value: "42".to_string(), position: Position { line: 1, column: 1 } },
        Token { token_type: TokenType::EOF, value: "".to_string(), position: Position { line: 1, column: 3 } },
    ];

    let mut parser = Parser::new(tokens);
    let mut symbol_table = SymbolTable::new(); // Initialize symbol table

    let result = parser.parse_statement(&mut symbol_table);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Unexpected token Some(Token { token_type: Number, value: \"42\", position: Position { line: 1, column: 1 } }) at position Position { line: 1, column: 1 }. Expected 'let' to start a declaration."
    );
}
