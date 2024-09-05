use std::fs;
mod lexer;
mod util;
mod models;
mod repr;
mod errors;
use crate::models::TokenType;
fn main() {
    let input =
        fs::read_to_string("input/main.cru").expect("should have been able to read the file");
    println!("{input}");
    let tokens = lexer::lexer(&input);
    for token in tokens{
        if token.token_type == TokenType::Unknown{
            errors::invalid_char(token)
        }
        }
    repr::interactive_lexer();
}
