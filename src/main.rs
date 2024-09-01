use std::fs;
mod lexer;
mod models; 
fn main() {
        let input = fs::read_to_string("input/main.cru").expect("should have been able to read the file");
        println!("{input}");
        let tokens = lexer::lexer(&input);
        for token in tokens{
                println!("{:?}",token);
        }
        
}
