
use crate::models::{TokenType, Token, Position};
use crate::ast::{AST, ASTNode};
use crate::symbol_table::{SymbolTable, SymbolType, SymbolValue};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    pub current_token: Option<Token>,
    pub position: Position,
    pub symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_index: 0,
            current_token: None,
            position: Position { line: 1, column: 1 },
            symbol_table: SymbolTable::new(),
        };
        parser.advance();
        parser
    }

    pub fn parse_program(&mut self) -> Result<(Vec<AST>, SymbolTable), String> {
        let mut asts = Vec::new();

        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::EOF {
                break;
            }
            
            let ast = self.parse_statement()?;
            self.symbol_table.print(); 
            asts.push(ast);
        }

        Ok((asts, self.symbol_table.clone()))
    }
    pub fn parse_assignment(&mut self, variable: String) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Assign) {
            return Err(format!("Expected ':=' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ':='
    
        let expression = self.parse_expression()?;
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ';'
    
        Ok(AST::new(ASTNode::Assignment {
            variable,
            expression: Box::new(expression),
        }))
    }
    pub fn parse_statement(&mut self) -> Result<AST, String> {
        match self.current_token {
            Some(ref token) if token.token_type == TokenType::Let => self.parse_let_decl(),
            Some(ref token) if token.token_type == TokenType::Print => self.parse_print(),
            Some(ref token) if token.token_type == TokenType::If => self.parse_if_statement(),
            Some(ref token) if token.token_type == TokenType::Loop => self.parse_loop(),
            Some(ref token) if token.token_type == TokenType::Break => self.parse_break(),
            Some(ref token) if token.token_type == TokenType::Identifier => {
                let identifier = token.value.clone();
                self.advance();
                if self.current_token_is(TokenType::Assign) {
                    self.parse_assignment(identifier)
                } else if self.current_token_is(TokenType::Dot) {
                    self.advance();
                    if self.current_token_is(TokenType::Push) {
                        self.parse_push(identifier)
                    } else if self.current_token_is(TokenType::Pop) {
                        self.parse_pop(identifier)
                    } else if self.current_token_is(TokenType::Identifier) {
                        let method_name = self.current_token.as_ref().unwrap().value.clone();
                        self.advance(); // Consume the method name

                        match method_name.as_str() {
                            "fetch" => self.parse_fetch(AST::new(ASTNode::Identifier(identifier))),
                            "len" => self.parse_len(AST::new(ASTNode::Identifier(identifier))),
                            _ => Err(format!(
                                "Unexpected method '{}' at position {:?}. Expected 'fetch' or 'len'.",
                                method_name, self.position
                            )),
                        }
                    } else {
                        Err(format!(
                            "Unexpected token {:?} at position {:?}. Expected 'push', 'pop', or a method name.",
                            self.current_token, self.position
                        ))
                    }
                } else {
                    Err(format!(
                        "Unexpected token {:?} at position {:?}. Expected ':=' for assignment or '.' for method call.",
                        self.current_token, self.position
                    ))
                }
            }
            _ => Err(format!(
                "Unexpected token {:?} at position {:?}. Expected 'let', 'stdout', 'if', 'loop', 'break', or an identifier.",
                self.current_token, self.position
            )),
        }
    }
    fn parse_len(&mut self, list_expression: AST) -> Result<AST, String> {
        if self.current_token_is(TokenType::LeftParen) {
            self.advance(); // Consume '('

            if self.current_token_is(TokenType::RightParen) {
                self.advance(); // Consume ')'
                return Ok(AST::new(ASTNode::Len {
                    list: Box::new(list_expression),
                }));
            } else {
                return Err(format!(
                    "Expected ')' after 'len' at position {:?}. Found {:?}",
                    self.position, self.current_token
                ));
            }
        } else {
            return Err(format!(
                "Expected '(' after 'len' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }
    }

    pub fn parse_push(&mut self, list_name: String) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Push) {
            return Err(format!("Expected 'push' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'push'
    
        if !self.current_token_is(TokenType::LeftParen) {
            return Err(format!("Expected '(' after 'push' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume '('
    
        let value = self.parse_expression()?;
    
        if !self.current_token_is(TokenType::RightParen) {
            return Err(format!("Expected ')' after value expression at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ')'
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ';'
    
        Ok(AST::new(ASTNode::Push {
            list: Box::new(AST::new(ASTNode::Identifier(list_name))),
            value: Box::new(value),
        }))
    }
    pub fn parse_break(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Break) {
            return Err(format!("Expected 'break' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'break'
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' after 'break' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ';'
    
        Ok(AST::new(ASTNode::Break))
    }
    pub fn parse_loop(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Loop) {
            return Err(format!("Expected 'loop' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'loop'
    
        if !self.current_token_is(TokenType::LeftBrace) {
            return Err(format!("Expected '{{' after 'loop' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume '{'
    
        let mut body_statements = Vec::new();
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            body_statements.push(statement);
        }
    
        if !self.current_token_is(TokenType::RightBrace) {
            return Err(format!("Expected '}}' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume '}'
    
        Ok(AST::new(ASTNode::Loop {
            body: Box::new(AST::new(ASTNode::List(body_statements))),
        }))
    }
    pub fn parse_if_statement(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::If) {
            return Err(format!("Expected 'if' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'if'
    
        if !self.current_token_is(TokenType::LeftParen) {
            return Err(format!("Expected '(' after 'if' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume '('
    
        let condition = self.parse_expression()?;
    
        if !self.current_token_is(TokenType::RightParen) {
            return Err(format!("Expected ')' after condition at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ')'
    
        if !self.current_token_is(TokenType::Then) {
            return Err(format!("Expected 'then' after 'if' condition at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'then'
    
        let then_branch = self.parse_statement()?;
    
        let mut else_branch = None;
        if self.current_token_is(TokenType::Else) {
            self.advance(); // Consume 'else'
            else_branch = Some(self.parse_statement()?);
        }
    
        if !self.current_token_is(TokenType::End) {
            return Err(format!("Expected 'end' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'end'
    
        Ok(AST::new(ASTNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }))
    }
    pub fn parse_pop(&mut self, list_name: String) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Pop) {
            return Err(format!("Expected 'pop' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume 'pop'
    
        if !self.current_token_is(TokenType::LeftParen) {
            return Err(format!("Expected '(' after 'pop' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume '('
    
        if !self.current_token_is(TokenType::RightParen) {
            return Err(format!("Expected ')' after 'pop' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ')'
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); // Consume ';'
    
        Ok(AST::new(ASTNode::Pop {
            list: Box::new(AST::new(ASTNode::Identifier(list_name))),
        }))
    }
    pub fn parse_let_decl(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Let) {
            return Err(format!("Expected 'let' at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        self.advance(); 
        if !self.current_token_is(TokenType::Identifier) {
            return Err(format!("Expected identifier at position {:?}. Found {:?}", self.position, self.current_token));
        }
    
        let variable = self.current_token.as_ref().unwrap().value.clone();
        if self.symbol_table.lookup(&variable).is_some() {
            return Err(format!(
                "Variable '{}' already declared at position {:?}.",
                variable, self.position
            ));
        }
    
        self.advance(); 
    
        let symbol_type = match self.current_token {
            Some(ref token) if token.token_type == TokenType::IntType => SymbolType::Int,
            Some(ref token) if token.token_type == TokenType::FloatType => SymbolType::Float,
            Some(ref token) if token.token_type == TokenType::BoolType => SymbolType::Boolean,
            Some(ref token) if token.token_type == TokenType::ListIntType => SymbolType::List(Box::new(SymbolType::Int)),
            Some(ref token) if token.token_type == TokenType::ListFloatType => SymbolType::List(Box::new(SymbolType::Float)),
            Some(ref token) if token.token_type == TokenType::ListBoolType => SymbolType::List(Box::new(SymbolType::Boolean)),
            Some(ref token) if token.token_type == TokenType::Colon => {
                self.advance(); // Advance to the next token
                match self.current_token {
                    Some(ref token) if token.token_type == TokenType::IntType => SymbolType::List(Box::new(SymbolType::Int)),
                    Some(ref token) if token.token_type == TokenType::FloatType => SymbolType::List(Box::new(SymbolType::Float)),
                    Some(ref token) if token.token_type == TokenType::BoolType => SymbolType::List(Box::new(SymbolType::Boolean)),
                    _ => return Err(format!(
                        "Expected list type 'int', 'float', or 'bool' at position {:?}. Found {:?}.",
                        self.position, self.current_token
                    )),
                }
            },
            _ => return Err(format!(
                "Expected type 'int', 'float', 'bool', or list at position {:?}. Found {:?}.",
                self.position, self.current_token
            )),
        };
    
        self.advance(); 
        if !self.current_token_is(TokenType::Assign) {
            return Err(format!(
                "Expected ':=' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }
    
        self.advance();
        let mut expression = self.parse_expression()?;
    
        // Check for fetch method call
        if self.current_token_is(TokenType::Dot) {
            self.advance(); // Consume the dot
        
            if self.current_token_is(TokenType::Identifier) {
                let method_name = self.current_token.clone().unwrap().value.clone();
                self.advance(); // Consume the method name
        
                match method_name.as_str() {
                    "fetch" => {
                        expression = self.parse_fetch(expression)?;
                    },
                    "len" => {
                        expression = self.parse_len(expression)?;
                    },
                    _ => {
                        return Err(format!(
                            "Unknown method '{}' at position {:?}.",
                            method_name, self.position
                        ));
                    }
                }
            } else {
                return Err(format!(
                    "Expected method name after '.' at position {:?}. Found {:?}",
                    self.position, self.current_token
                ));
            }
        }
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!(
                "Expected ';' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }
    
        self.advance();
    
        let expr_type = self.infer_type(&expression)?;
        if expr_type != symbol_type {
            return Err(format!(
                "Type mismatch: cannot assign expression of type {:?} to variable of type {:?} at position {:?}.",
                expr_type, symbol_type, self.position
            ));
        }
    
        let value = self.evaluate_expression(&expression)?;
    
        self.symbol_table.insert(variable.clone(), symbol_type, value)
            .map_err(|e| format!("Error inserting symbol into symbol table: {}", e))?;
    
        Ok(AST::new(ASTNode::Assignment {
            variable,
            expression: Box::new(expression),
        }))
    }
    pub fn parse_fetch(&mut self, list: AST) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Dot) {
            return Err(format!("Expected '.' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Consume '.'

        if !self.current_token_is(TokenType::Fetch) {
            return Err(format!("Expected 'fetch' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Consume 'fetch'

        if !self.current_token_is(TokenType::LeftParen) {
            return Err(format!("Expected '(' after 'fetch' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Consume '('

        let index = self.parse_expression()?;

        if !self.current_token_is(TokenType::RightParen) {
            return Err(format!("Expected ')' after index expression at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Consume ')'

        Ok(AST::new(ASTNode::Fetch {
            list: Box::new(list),
            index: Box::new(index),
        }))
    }

    pub fn parse_expression(&mut self) -> Result<AST, String> {
        let mut left = self.parse_term()?;
        let mut left_type = self.infer_type(&left)?;

        while self.current_token_is(TokenType::Operator)
            || self.current_token_is(TokenType::LessThan)
            || self.current_token_is(TokenType::GreaterThan)
            || self.current_token_is(TokenType::Equal)
            || self.current_token_is(TokenType::NotEqual)
            || self.current_token_is(TokenType::And)
            || self.current_token_is(TokenType::Or)
        {
            let operator = self.current_token.as_ref().unwrap().value.clone();
            self.advance();

            let right = self.parse_term()?;
            let right_type = self.infer_type(&right)?;

            if operator == "==" || operator == "!=" || operator == ">" || operator == "<" || operator == ">=" || operator == "<=" {
                if left_type != right_type {
                    return Err(format!(
                        "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                        operator, left_type, right_type, self.position
                    ));
                }
            } else if operator == "&&" || operator == "||" {
                if left_type != SymbolType::Boolean || right_type != SymbolType::Boolean {
                    return Err(format!(
                        "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                        operator, left_type, right_type, self.position
                    ));
                }
            } else if left_type != right_type {
                return Err(format!(
                    "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                    operator, left_type, right_type, self.position
                ));
            }

            left = AST::new(ASTNode::BinaryOperation {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            });

            left_type = self.infer_type(&left)?;
        }

        Ok(left)
    }

    pub fn parse_list(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::LeftBracket) {
            return Err(format!("Expected '[' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); // Consume '['

        let mut elements = Vec::new();
        while !self.current_token_is(TokenType::RightBracket) {
            let element = self.parse_expression()?;
            elements.push(element);

            if self.current_token_is(TokenType::Comma) {
                self.advance(); // Consume ','
            } else if !self.current_token_is(TokenType::RightBracket) {
                return Err(format!("Expected ',' or ']' at position {:?}. Found {:?}", self.position, self.current_token));
            }
        }

        self.advance(); // Consume ']'

        Ok(AST::new(ASTNode::List(elements)))
    }
    pub fn parse_term(&mut self) -> Result<AST, String> {
        let mut left = self.parse_factor()?;
        let mut left_type = self.infer_type(&left)?;
    
        while self.current_token_is(TokenType::Operator)
            && (self.current_token.as_ref().unwrap().value == "*"
                || self.current_token.as_ref().unwrap().value == "/")
        {
            let operator = self.current_token.as_ref().unwrap().value.clone();
            self.advance();
            let right = self.parse_factor()?;
            let right_type = self.infer_type(&right)?;
    
            if left_type != right_type {
                return Err(format!(
                    "Type mismatch: cannot perform '{}' operation between {:?} and {:?} at position {:?}.",
                    operator, left_type, right_type, self.position
                ));
            }
    
            left = AST::new(ASTNode::BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
            left_type = self.infer_type(&left)?;
        }
    
        Ok(left)
    }
 
   

/*    pub fn parse_assign_expr(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Assign) {
            return Err(format!(
                "Expected ':=' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }
    
        self.advance();
        let expr = self.parse_expression()?;
    
        if !self.current_token_is(TokenType::Termination) {
            return Err(format!(
                "Expected ';' at position {:?}. Found {:?}",
                self.position, self.current_token
            ));
        }
    
        self.advance();
        Ok(expr)
    } */
    pub fn parse_factor(&mut self) -> Result<AST, String> {
        match self.current_token {
            Some(ref token) if token.token_type == TokenType::Number => {
                let value = token.value.parse::<i64>()
                    .map_err(|_| format!("Invalid integer format at position {:?}.", self.position))?;
                self.advance();
                Ok(AST::new(ASTNode::Int(value)))
            },
            Some(ref token) if token.token_type == TokenType::FloatNumber => {
                let value = token.value.parse::<f64>()
                    .map_err(|_| format!("Invalid float format at position {:?}.", self.position))?;
                self.advance();
                Ok(AST::new(ASTNode::Float(value)))
            },
            Some(ref token) if token.token_type == TokenType::Boolean => {
                let value = token.value == "true";
                self.advance();
                Ok(AST::new(ASTNode::Boolean(value)))
            },
            Some(ref token) if token.token_type == TokenType::Identifier => {
                let value = token.value.clone();
                self.advance();
                let mut expression = AST::new(ASTNode::Identifier(value));

                // Check for method calls (fetch or len)
                if self.current_token_is(TokenType::Dot) {
                    self.advance(); // Consume the dot
                
                    match self.current_token {
                        Some(ref token) if token.token_type == TokenType::Fetch => {
                            self.advance(); // Consume 'fetch'
                            expression = self.parse_fetch(expression)?;
                        },
                        Some(ref token) if token.token_type == TokenType::Len => {
                            self.advance(); // Consume 'len'
                            expression = self.parse_len(expression)?;
                        },
                        Some(ref token) => {
                            return Err(format!(
                                "Unknown method '{:?}' at position {:?}.",
                                token.token_type, self.position
                            ));
                        },
                        None => {
                            return Err(format!(
                                "Expected method name after '.' at position {:?}. Found None",
                                self.position
                            ));
                        }
                    }
                }

                Ok(expression)
            },
            Some(ref token) if token.token_type == TokenType::LeftParen => {
                self.advance(); 
                let expr = self.parse_expression()?;
                if !self.current_token_is(TokenType::RightParen) {
                    return Err(format!("Expected ')' at position {:?}. Found {:?}", self.position, self.current_token));
                }
                self.advance(); 
                Ok(expr)
            },
            Some(ref token) if token.token_type == TokenType::LeftBracket => {
                self.parse_list()
            },
            _ => Err(format!("Unexpected token {:?} at position {:?}. Expected a number, float, identifier, boolean, or list.", self.current_token, self.position)),
        }
    }

    pub fn parse_print(&mut self) -> Result<AST, String> {
        if !self.current_token_is(TokenType::Print) {
            return Err(format!("Expected 'stdout' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
        if !self.current_token_is(TokenType::LeftParen) || self.current_token.as_ref().unwrap().value != "(" {
            return Err(format!("Expected '(' after 'stdout' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
        let expression = self.parse_expression()?;

        if !self.current_token_is(TokenType::RightParen) || self.current_token.as_ref().unwrap().value != ")" {
            return Err(format!("Expected ')' after expression at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 

        if !self.current_token_is(TokenType::Termination) {
            return Err(format!("Expected ';' at position {:?}. Found {:?}", self.position, self.current_token));
        }

        self.advance(); 
Ok(AST::new(ASTNode::Print(Box::new(expression))))
    }

    pub fn infer_type(&self, ast: &AST) -> Result<SymbolType, String> {
        match &ast.node {
            ASTNode::Int(_) => Ok(SymbolType::Int),
            ASTNode::Float(_) => Ok(SymbolType::Float),
            ASTNode::Boolean(_) => Ok(SymbolType::Boolean),
            ASTNode::Identifier(name) => self.symbol_table.lookup(name)
                .map(|symbol| symbol.symbol_type.clone())
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            ASTNode::List(elements) => {
                if elements.is_empty() {
                    return Err("Cannot infer type of empty list.".to_string());
                }
                let first_type = self.infer_type(&elements[0])?;
                for element in elements.iter().skip(1) {
                    let element_type = self.infer_type(element)?;
                    if element_type != first_type {
                        return Err(format!("Type mismatch in list elements: {:?} and {:?}.", first_type, element_type));
                    }
                }
                Ok(SymbolType::List(Box::new(first_type)))
            }
            ASTNode::BinaryOperation { operator, left, right } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
    
                match operator.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if (left_type == SymbolType::Int && right_type == SymbolType::Int) ||
                           (left_type == SymbolType::Float && right_type == SymbolType::Float) {
                            Ok(left_type)
                        } else {
                            Err(format!("Type mismatch: cannot perform '{}' operation between {:?} and {:?}.", operator, left_type, right_type))
                        }
                    }
                    "==" | "!=" | ">" | "<" | ">=" | "<=" => Ok(SymbolType::Boolean),
                    "&&" | "||" => {
                        if left_type == SymbolType::Boolean && right_type == SymbolType::Boolean {
                            Ok(SymbolType::Boolean)
                        } else {
                            Err(format!("Type mismatch: cannot perform '{}' operation between {:?} and {:?}.", operator, left_type, right_type))
                        }
                    }
                    _ => Err(format!("Unknown operator: {}", operator)),
                }
            }
            ASTNode::Push { list, value } => {
                let list_type = self.infer_type(list)?;
                let value_type = self.infer_type(value)?;
    
                if let SymbolType::List(element_type) = list_type {
                    if *element_type == value_type {
                        Ok(SymbolType::List(element_type))
                    } else {
                        Err(format!("Type mismatch: cannot push value of type {:?} to list of type {:?}.", value_type, element_type))
                    }
                } else {
                    Err(format!("Type mismatch: push operation can only be performed on lists, found {:?}.", list_type))
                }
            }
            ASTNode::Fetch { list, index } => {
                let list_type = self.infer_type(list)?;
                let index_type = self.infer_type(index)?;
    
                if index_type != SymbolType::Int {
                    return Err(format!("Type mismatch: index must be of type Int, found {:?}.", index_type));
                }
    
                if let SymbolType::List(element_type) = list_type {
                    Ok(*element_type)
                } else {
                    Err(format!("Type mismatch: fetch operation can only be performed on lists, found {:?}.", list_type))
                }
            }
            ASTNode::Len { list } => {
                let list_type = self.infer_type(list)?;
    
                if let SymbolType::List(_) = list_type {
                    Ok(SymbolType::Int)
                } else {
                    Err(format!("Type mismatch: len operation can only be performed on lists, found {:?}.", list_type))
                }
            }
            _ => Err(format!("Unknown AST node: {:?}", ast.node)),
        }
    }
    pub fn evaluate_expression(&self, ast: &AST) -> Result<SymbolValue, String> {
        match &ast.node {
            ASTNode::Int(value) => Ok(SymbolValue::Int(*value)),
            ASTNode::Float(value) => Ok(SymbolValue::Float(*value)),
            ASTNode::Boolean(value) => Ok(SymbolValue::Boolean(*value)),
            ASTNode::Identifier(ref name) => {
                self.symbol_table.lookup(name)
                    .map(|symbol| symbol.value.clone())
                    .ok_or_else(|| format!("Undefined variable '{}' at position {:?}", name, self.position))
            },
            ASTNode::List(ref elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate_expression(element)?);
                }
                Ok(SymbolValue::List(values))
            }
            ASTNode::BinaryOperation { ref left, ref right, ref operator } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
    
                match operator.as_str() {
                    "+" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l + r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l + r)),
                        _ => Err(format!("Type mismatch for '+' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "-" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l - r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l - r)),
                        _ => Err(format!("Type mismatch for '-' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "*" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l * r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l * r)),
                        _ => Err(format!("Type mismatch for '*' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "/" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Int(l / r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Float(l / r)),
                        _ => Err(format!("Type mismatch for '/' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "==" => Ok(SymbolValue::Boolean(left_value == right_value)),
                    "!=" => Ok(SymbolValue::Boolean(left_value != right_value)),
                    ">" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Boolean(l > r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Boolean(l > r)),
                        _ => Err(format!("Type mismatch for '>' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "<" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Boolean(l < r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Boolean(l < r)),
                        _ => Err(format!("Type mismatch for '<' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    ">=" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Boolean(l >= r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Boolean(l >= r)),
                        _ => Err(format!("Type mismatch for '>=' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "<=" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Int(l), SymbolValue::Int(r)) => Ok(SymbolValue::Boolean(l <= r)),
                        (SymbolValue::Float(l), SymbolValue::Float(r)) => Ok(SymbolValue::Boolean(l <= r)),
                        _ => Err(format!("Type mismatch for '<=' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "&&" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Boolean(l), SymbolValue::Boolean(r)) => Ok(SymbolValue::Boolean(l && r)),
                        _ => Err(format!("Type mismatch for '&&' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    "||" => match (left_value.clone(), right_value.clone()) {
                        (SymbolValue::Boolean(l), SymbolValue::Boolean(r)) => Ok(SymbolValue::Boolean(l || r)),
                        _ => Err(format!("Type mismatch for '||' operation. Left: {:?}, Right: {:?}", left_value, right_value)),
                    },
                    _ => Err(format!("Unknown operator '{}' in binary operation.", operator)),
                }
            },
            ASTNode::Fetch { ref list, ref index } => {
                let list_value = self.evaluate_expression(list)?;
                let index_value = self.evaluate_expression(index)?;
    
                if let SymbolValue::List(elements) = list_value {
                    if let SymbolValue::Int(index) = index_value {
                        if index >= 0 && (index as usize) < elements.len() {
                            Ok(elements[index as usize].clone())
                        } else {
                            Err(format!("Index {} out of bounds.", index))
                        }
                    } else {
                        Err("Index must be an integer.".to_string())
                    }
                } else {
                    Err("Fetch operation can only be performed on lists.".to_string())
                }
            }
            ASTNode::Push { list, value } => {
                let list_value = self.evaluate_expression(list)?;
                let value_value = self.evaluate_expression(value)?;
    
                if let SymbolValue::List(mut elements) = list_value {
                    elements.push(value_value);
                    Ok(SymbolValue::List(elements))
                } else {
                    Err("Push operation can only be performed on lists.".to_string())
                }
            }
            ASTNode::Len { ref list } => {
                let list_value = self.evaluate_expression(list)?;

                if let SymbolValue::List(elements) = list_value {
                    Ok(SymbolValue::Int(elements.len() as i64))
                } else {
                    Err("Len operation can only be performed on lists.".to_string())
                }
            }
            _ => Err(format!("Cannot evaluate AST node {:?}", ast.node)),
        }
    }
    pub fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_token = Some(self.tokens[self.current_index].clone());
            self.current_index += 1;
        } else {
            self.current_token = None;
        }
    }

    pub fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map(|token| token.token_type == token_type)
            .unwrap_or(false)
    }
}
