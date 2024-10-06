use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Int,
    Float,
    Boolean,
    List(Box<SymbolType>), // Add list type
    Void, // Add void type
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolValue {
    Int(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<SymbolValue>), // Add list value
    Void, // Add void value
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub value: SymbolValue,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn insert(&mut self, name: String, symbol_type: SymbolType, value: SymbolValue) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            Err(format!("Symbol '{}' already declared", name))
        } else {
            current_scope.insert(name, Symbol { symbol_type, value });
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn push(&mut self, list_name: &str, value: SymbolValue) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(list_name) {
                if let SymbolValue::List(ref mut list) = symbol.value {
                    list.push(value);
                    return Ok(());
                } else {
                    return Err(format!("Symbol '{}' is not a list", list_name));
                }
            }
        }
        Err(format!("Symbol '{}' not found", list_name))
    }

    pub fn pop(&mut self, list_name: &str) -> Result<SymbolValue, String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(list_name) {
                if let SymbolValue::List(ref mut list) = symbol.value {
                    if let Some(value) = list.pop() {
                        return Ok(value);
                    } else {
                        return Err(format!("List '{}' is empty", list_name));
                    }
                } else {
                    return Err(format!("Symbol '{}' is not a list", list_name));
                }
            }
        }
        Err(format!("Symbol '{}' not found", list_name))
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn print(&self) {
        for (i, scope) in self.scopes.iter().enumerate() {
            println!("Scope {}: {:?}", i, scope);
        }
    }
}