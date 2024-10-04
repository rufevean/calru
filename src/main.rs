mod errors;
mod lexer;
mod models;
mod parser;
mod symbol_table;
//mod ir;
mod ast;
mod interpreter;

use crate::parser::Parser;
//use crate::ir::generator::generate_ir;
//use crate::ir::instruction::write_asm_file;
use crate::interpreter::Interpreter;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("input/main.cru").expect("should have been able to read the file");
    //    println!("{}", input);

    match lexer::lexer(&input) {
        Ok(tokens) => {

            let mut parser = Parser::new(tokens);
            match parser.parse_program() {
                Ok((asts, symbol_table)) => {
                    // Ensure parse_program returns both
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
        Err(e) => eprintln!("Lexing failed: {}", e),
    }
}
