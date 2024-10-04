use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Int,
    Float,
    Boolean,
    List(Box<SymbolType>), // Add list type
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolValue {
    Int(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<SymbolValue>), // Add list value
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub value: SymbolValue,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol_type: SymbolType, value: SymbolValue) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            Err(format!("Symbol '{}' already declared", name))
        } else {
            self.symbols.insert(name, Symbol { symbol_type, value });
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn push(&mut self, list_name: &str, value: SymbolValue) -> Result<(), String> {
        if let Some(symbol) = self.symbols.get_mut(list_name) {
            if let SymbolValue::List(ref mut list) = symbol.value {
                list.push(value);
                Ok(())
            } else {
                Err(format!("Symbol '{}' is not a list", list_name))
            }
        } else {
            Err(format!("Symbol '{}' not found", list_name))
        }
    }

    pub fn pop(&mut self, list_name: &str) -> Result<SymbolValue, String> {
        if let Some(symbol) = self.symbols.get_mut(list_name) {
            if let SymbolValue::List(ref mut list) = symbol.value {
                if let Some(value) = list.pop() {
                    Ok(value)
                } else {
                    Err(format!("List '{}' is empty", list_name))
                }
            } else {
                Err(format!("Symbol '{}' is not a list", list_name))
            }
        } else {
            Err(format!("Symbol '{}' not found", list_name))
        }
    }

    pub fn print(&self) {
        for (_name, symbol) in &self.symbols {
            let _value_str = match &symbol.value {
                SymbolValue::Int(v) => v.to_string(),
                SymbolValue::Float(v) => v.to_string(),
                SymbolValue::Boolean(v) => v.to_string(),
                SymbolValue::List(v) => format!("{:?}", v), // Handle list values
            };
        }
    }
}