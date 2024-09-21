
mod lexer;
mod models;
mod errors;
mod parser;
mod symbol_table;
//mod ir;
mod interpreter;
mod ast;

use crate::models::TokenType;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;
//use crate::ir::generator::generate_ir;
//use crate::ir::instruction::write_asm_file;
use crate::interpreter::Interpreter;
use crate::ast::AST; // Ensure correct import
use std::fs;

fn main() {
    let input = fs::read_to_string("input/main.cru")
        .expect("should have been able to read the file");
    println!("{}", input);

    let tokens = lexer::lexer(&input);

    for token in &tokens {
        if token.token_type == TokenType::Unknown {
            errors::invalid_char(token.clone());
        }
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok((asts, symbol_table)) => { // Ensure parse_program returns both
            
            /*let mut all_instructions = Vec::new();

            for ast in &asts { 
                let instructions = generate_ir(ast);
                all_instructions.extend(instructions.clone());

                for instruction in &instructions {
                    println!("{:?}", instruction);
                }
            }
            
            if let Err(e) = write_asm_file(&all_instructions, "output.asm") {
                eprintln!("Failed to write assembly file: {}", e);
            }
            */ 

            let mut interpreter = Interpreter::new(symbol_table);

            for ast in asts {
                if let Err(e) = interpreter.run(vec![ast]) {
                    eprintln!("Execution error: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
