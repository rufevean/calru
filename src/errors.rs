use crate::models::Token ;

pub fn invalid_char(token : Token) {
    panic!("Lexer Error : Invalid character `{}` at line `{:?}`  and column `{:?}` ",token.value,token.position.line,token.position.column)
} 

pub fn invalid_variable_name(num : String , line : usize, column : usize) {
    panic!(
        "Lexer Error : Invalid variable name `{}` at line {line} and column {column}  , Variable names cannot be initiated with a number",num
    )
}
