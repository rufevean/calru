
mod lexer;
mod models;
mod errors;
mod parser;
mod symbol_table;
mod ir;
mod interpreter;
mod ast;

use crate::models::TokenType;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;
use crate::ir::generator::generate_ir;
use crate::ir::instruction::write_asm_file;
use crate::interpreter::Interpreter;
use crate::ast::AST; // Ensure correct import
use std::fs;

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
    }

    // Initialize the parser
    let mut parser = Parser::new(tokens);

    // Parse the tokens into an AST and print the AST
    match parser.parse_program() {
        Ok(asts) => {
            let mut all_instructions = Vec::new();

            for ast in &asts { // Use reference to avoid unnecessary cloning
                // Generate IR instructions
                let instructions = generate_ir(ast);
                all_instructions.extend(instructions.clone());

                // Print instructions (for debugging purposes)
                for instruction in &instructions {
                    println!("{:?}", instruction);
                }
            }

            // Write instructions to an assembly file
            if let Err(e) = write_asm_file(&all_instructions, "output.asm") {
                eprintln!("Failed to write assembly file: {}", e);
            }

            // Initialize the interpreter
            let mut interpreter = Interpreter::new();

            // Execute the AST
            for ast in asts {
                if let Err(e) = interpreter.run(vec![ast]) {
                    eprintln!("Execution error: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
