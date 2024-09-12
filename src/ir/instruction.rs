
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum IRInstruction {
    Mov { dest: String, src: String },
    Add { dest: String, src: String },
    Sub { dest: String, src: String },
    Mul { dest: String, src: String },
    Div { dest: String, src: String },
    Print { operand: String },
}

pub fn write_asm_file(instructions: &[IRInstruction], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    let mut variables = HashMap::new();
    let mut current_var_id = 0;

    for instruction in instructions {
        match instruction {
            IRInstruction::Mov { dest, src } => {
                if !variables.contains_key(dest) && !dest.starts_with("R") {
                    variables.insert(dest.clone(), format!("v{}", current_var_id));
                    current_var_id += 1;
                }
                if !variables.contains_key(src) && !src.starts_with("R") && src.parse::<i64>().is_err() {
                    variables.insert(src.clone(), format!("v{}", current_var_id));
                    current_var_id += 1;
                }
            }
            IRInstruction::Add { dest, src }
            | IRInstruction::Sub { dest, src }
            | IRInstruction::Mul { dest, src }
            | IRInstruction::Div { dest, src } => {
                if !variables.contains_key(dest) && !dest.starts_with("R") {
                    variables.insert(dest.clone(), format!("v{}", current_var_id));
                    current_var_id += 1;
                }
                if !variables.contains_key(src) && !src.starts_with("R") && src.parse::<i64>().is_err() {
                    variables.insert(src.clone(), format!("v{}", current_var_id));
                    current_var_id += 1;
                }
            }
            IRInstruction::Print { operand } => {
                if !variables.contains_key(operand) && !operand.starts_with("R") {
                    variables.insert(operand.clone(), format!("v{}", current_var_id));
                    current_var_id += 1;
                }
            }
        }
    }

    writeln!(file, "section .data")?;
    writeln!(file, "msg db 'Result: ', 0")?; 
    writeln!(file, "buffer db 20 dup(0)")?; 

    for (var_name, asm_name) in &variables {
        writeln!(file, "{} dq 0", asm_name)?;
    }

    writeln!(file, "section .text")?;
    writeln!(file, "global _start")?;
    writeln!(file, "_start:")?;

    for instruction in instructions {
        let asm_line = match instruction {
            IRInstruction::Mov { dest, src } => {
                let dest_op = convert_to_register_or_memory(dest, &variables);
                let src_op = convert_to_register_or_memory(src, &variables);
                
                if src.parse::<i64>().is_ok() {
                    format!("mov {}, {}\n", dest_op, src_op) 
                } else {
                    format!("mov {}, qword {}\n", dest_op, src_op) 
                }
            },
            IRInstruction::Add { dest, src } => {
                let (dest_op, src_op) = (convert_to_register_or_memory(dest, &variables), convert_to_register_or_memory(src, &variables));
                format!("add {}, {}\n", dest_op, src_op)
            },
            IRInstruction::Sub { dest, src } => {
                let (dest_op, src_op) = (convert_to_register_or_memory(dest, &variables), convert_to_register_or_memory(src, &variables));
                format!("sub {}, {}\n", dest_op, src_op)
            },
            IRInstruction::Mul { dest, src } => {
                let (dest_op, src_op) = (convert_to_register_or_memory(dest, &variables), convert_to_register_or_memory(src, &variables));
                format!("imul {}, {}\n", dest_op, src_op)
            },
            IRInstruction::Div { dest, src } => {
                let dest_op = convert_to_register_or_memory(dest, &variables);
                let src_op = convert_to_register_or_memory(src, &variables);
                format!("mov rax, {}\ncqo\nidiv {}\n", dest_op, src_op)
            },
            IRInstruction::Print { operand } => {
                let operand_mem = convert_to_register_or_memory(operand, &variables);
                format!(
                    "mov rsi, {}\n\
                     call print_integer\n\
                     mov rsi, msg\n\
                     mov rdx, 8\n\
                     mov rax, 1\n\
                     mov rdi, 1\n\
                     syscall\n",
                    operand_mem
                )
            },
        };
        file.write_all(asm_line.as_bytes())?;
    }
    Ok(())
}

fn convert_to_register_or_memory(symbol: &str, variables: &HashMap<String, String>) -> String {
    if let Some(var) = variables.get(symbol) {
        return format!("[{}]", var);
    }
    match symbol {
        "R0" => "rax".to_string(),
        "R1" => "rbx".to_string(),
        _ => symbol.to_string(),
    }
}
