

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Int,
    Float,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol_type: SymbolType) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            Err(format!("Symbol '{}' already declared", name))
        } else {
            self.symbols.insert(name, symbol_type);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolType> {
        self.symbols.get(name)
    }
}
