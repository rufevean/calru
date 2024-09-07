
use crate::lexer::lexer;
use crate::models::{TokenType, Token};
use crate::parser::Parser;
use crate::errors;
use std::io::{self, Write};

pub fn interactive_lexer() {
    println!("Welcome to the Lexer CLI. Enter your code and press Enter:");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            println!("Exiting the lexer. Goodbye!");
            break;
        }

        let tokens = lexer(trimmed_input);

        for token in &tokens {
            if token.token_type == TokenType::Unknown {
                errors::invalid_char(token.clone());
            }
            println!("{:?}", token);
        }

        let mut parser = Parser::new(tokens);

        match parser.parse_statement() {
            Ok(_) => println!("Parsing succeeded."),
            Err(e) => println!("Parsing failed: {}", e),
        }
    }
}
