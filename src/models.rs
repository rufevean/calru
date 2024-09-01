#[derive(Debug,Clone)]
pub struct Token{
    pub token_type : TokenType,
    pub value : String,
    pub position : Position 
}


#[derive(Debug,Clone,Copy)]
pub enum TokenType{
    Identifier,
    Number,
    Let,
    Comment,
    Operator,
    Keyword,
    Assign, 
    Unknown,
    EOF
}


#[derive (Debug,Clone,Copy)]
pub struct Position{
    pub line : usize,
    pub column : usize,
}

impl Token{

    pub fn new(token_type:TokenType,value:String,position:Position)-> Self{
        Token{
            token_type,
            value,
            position 
        }
    }
}
