
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
    let input = fs::read_to_string("input/main.cru")
        .expect("should have been able to read the file");
    println!("{}", input);

    let tokens = lexer::lexer(&input);

    for token in &tokens {
        if token.token_type == TokenType::Unknown {
            errors::invalid_char(token.clone());
        }
    }

    let mut parser = Parser::new(tokens);
    let mut symbol_table = SymbolTable::new();

    match parser.parse_statement(&mut symbol_table) {
        Ok(ast) => {
            let instructions = generate_ir(&ast);
            
            for instruction in instructions {
                println!("{:?}", instruction);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    repr::interactive_lexer(); 
}
