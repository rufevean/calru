use crate::lexer::lexer;

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

        for token in tokens {
            println!("{:?}", token); // Use {:?} for Debug or {} for Display
        }
    }
}
