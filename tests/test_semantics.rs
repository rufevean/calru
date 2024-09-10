
#[cfg(test)]
mod tests {
    use super::*;
    use calru::parser::Parser;
    use calru::models::{TokenType, Token, Position};
    use calru::ast::ASTNode;
    use calru::symbol_table::SymbolTable;
    use calru::ast::AST;

    fn create_tokens(tokens: Vec<(&str, TokenType)>) -> Vec<Token> {
        tokens.into_iter()
            .enumerate()
            .map(|(i, (value, token_type))| Token {
                token_type,
                value: value.to_string(),
                position: Position { line: 1, column: i + 1 },
            })
            .collect()
    }

    #[test]
    fn test_valid_let_declaration() {
        let tokens = create_tokens(vec![
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("1", TokenType::Number),
            (";", TokenType::Termination),
        ]);

        let mut parser = Parser::new(tokens);

        let expected_ast = AST::new(ASTNode::Assignment {
            variable: "variable1".to_string(),
            expression: Box::new(AST::new(ASTNode::Int(1))),
        });

        match parser.parse_statement(&mut SymbolTable::new()) {
            Ok(ast) => assert_eq!(ast, expected_ast),
            Err(e) => panic!("Parsing failed: {}", e),
        }
    }

    #[test]
    fn test_redeclaring_variable() {
        let tokens = create_tokens(vec![
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("1", TokenType::Number),
            (";", TokenType::Termination),
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("2", TokenType::Number),
            (";", TokenType::Termination),
        ]);

        let mut parser = Parser::new(tokens);
        let mut symbol_table = SymbolTable::new();

        parser.parse_statement(&mut symbol_table).unwrap();

        let err = parser.parse_statement(&mut symbol_table).err().unwrap();
        assert_eq!(err, "Variable 'variable1' already declared at position Position { line: 1, column: 8 }.");
    }

    #[test]
    fn test_type_mismatch() {
        let tokens = create_tokens(vec![
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("1.5", TokenType::FloatNumber),
            (";", TokenType::Termination),
        ]);

        let mut parser = Parser::new(tokens);
        let mut symbol_table = SymbolTable::new();

        let err = parser.parse_statement(&mut symbol_table).err().unwrap();
        assert_eq!(err, "Type mismatch: cannot assign expression of type Float to variable of type Int at position Position { line: 1, column: 6 }.");
    }

    #[test]
    fn test_invalid_syntax() {
        let tokens = create_tokens(vec![
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("1", TokenType::Number),
        ]);

        let mut parser = Parser::new(tokens);

        // Missing termination ';'
        let err = parser.parse_statement(&mut SymbolTable::new()).err().unwrap();
        assert_eq!(err, "Expected ';' at position Position { line: 1, column: 5 }. Found None");
    }

    #[test]
    fn test_valid_expression() {
        let tokens = create_tokens(vec![
            ("let", TokenType::Let),
            ("variable1", TokenType::Identifier),
            (":int", TokenType::IntType),
            (":=", TokenType::Assign),
            ("1", TokenType::Number),
            ("+", TokenType::Operator),
            ("2", TokenType::Number),
            (";", TokenType::Termination),
        ]);

        let mut parser = Parser::new(tokens);

        let expected_ast = AST::new(ASTNode::Assignment {
            variable: "variable1".to_string(),
            expression: Box::new(AST::new(ASTNode::BinaryOperation {
                operator: "+".to_string(),
                left: Box::new(AST::new(ASTNode::Int(1))),
                right: Box::new(AST::new(ASTNode::Int(2))),
            })),
        });

        match parser.parse_statement(&mut SymbolTable::new()) {
            Ok(ast) => assert_eq!(ast, expected_ast),
            Err(e) => panic!("Parsing failed: {}", e),
        }
    }
}
