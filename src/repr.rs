
use crate::lexer::lexer;
use crate::models::TokenType;
use crate::parser::Parser;
use crate::errors;
use crate::symbol_table::SymbolTable;
use crate::ir::generator::generate_ir;
use std::io::{self, Write};

pub fn interactive_lexer() {
    println!("Welcome to the Calru. Enter your code and press Enter:");

    let mut symbol_table = SymbolTable::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            println!("Exiting the Repr. Goodbye!");
            break;
        }

        let tokens = lexer(trimmed_input);
        let mut parser = Parser::new(tokens);

        match parser.parse_statement() {
            Ok(ast) => {
                println!("Parsed AST:\n{}", ast);
                let instructions = generate_ir(&ast);

                println!("Generated Assembly Instructions:");
                for instruction in instructions {
                    println!("{:?}", instruction);
                }
            }
            Err(e) => println!("Parsing failed: {}", e),
        }
    }
}
