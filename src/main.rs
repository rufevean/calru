
use std::fs;

mod lexer;
mod util;
mod models;
mod ast;
mod parser;
mod repr;
mod errors;
mod symbol_table;
mod ir; 

use crate::models::TokenType;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;
use crate::ir::generator::generate_ir;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input/main.cru")
        .expect("should have been able to read the file");
    println!("{}", input);

    // Lex the input into tokens
    let tokens = lexer::lexer(&input);

    // Handle invalid tokens
    for token in &tokens {
        if token.token_type == TokenType::Unknown {
            errors::invalid_char(token.clone());
        }
        println!("{:?}", token);
    }

    // Initialize the parser and symbol table
    let mut parser = Parser::new(tokens);
    let mut symbol_table = SymbolTable::new();

    // Parse the tokens into an AST and print the AST

match parser.parse_program() {
    Ok(asts) => {
        for ast in asts {
            println!("Parsed AST:\n{}", ast);

            let instructions = generate_ir(&ast);

            println!("Generated Assembly Instructions:");
            for instruction in instructions {
                println!("{:?}", instruction);
            }
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}

    // Start the interactive lexer (optional)
    repr::interactive_lexer(); 
}
