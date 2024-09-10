
// src/ir/instruction.rs
#[derive(Debug, Clone)]
pub enum IRInstruction {
    Mov { dest: String, src: String },
    Add { dest: String, src: String },
    Sub { dest: String, src: String },
    Mul { dest: String, src: String },
    Div { dest: String, src: String },
    // Add more instructions as needed
}


impl IRInstruction {
    pub fn to_asm(&self) -> String {
        match self {
            IRInstruction::Mov { dest, src } => format!("MOV {}, {}", dest, src),
            IRInstruction::Add { dest, src } => format!("ADD {}, {}", dest, src),
            IRInstruction::Sub { dest, src } => format!("SUB {}, {}", dest, src),
            IRInstruction::Mul { dest, src } => format!("MUL {}, {}", dest, src),
            IRInstruction::Div { dest, src } => format!("DIV {}, {}", dest, src),
            // Handle more instructions if you add them
        }
    }
}
