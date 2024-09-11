
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Int,
    Float,
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub value: SymbolValue,
}

#[derive(Debug)]
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

    pub fn print(&self) {
        println!("Symbol Table:");
        for (name, symbol) in &self.symbols {
            let value_str = match &symbol.value {
                SymbolValue::Int(v) => v.to_string(),
                SymbolValue::Float(v) => v.to_string(),
            };
            println!("{}: {:?} = {}", name, symbol.symbol_type, value_str);
        }
    }
}
