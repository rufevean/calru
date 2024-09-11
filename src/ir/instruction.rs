
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

    // Write section headers
    writeln!(file, "section .data")?;
    writeln!(file, "msg db 'Result: ', 0")?; // Message prefix
    writeln!(file, "buffer db 20 dup(0)")?; // Buffer for integer to string conversion

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
                    format!("mov {}, {}\n", dest_op, src_op) // Move immediate value to register/memory
                } else {
                    format!("mov {}, qword {}\n", dest_op, src_op) // Move register/memory to register/memory
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

    // Write exit system call
    writeln!(file, "mov rax, 60")?; // System call number for sys_exit
    writeln!(file, "xor rdi, rdi")?; // Exit code 0
    writeln!(file, "syscall")?;

    // Define print_integer function
    writeln!(file, "print_integer:")?;
    writeln!(file, "    mov rbx, 10")?; // Divisor for division by 10
    writeln!(file, "    xor rdx, rdx")?; // Clear rdx for division
    writeln!(file, "    mov rdi, buffer + 19")?; // Point to the end of the buffer
    writeln!(file, "    mov byte [rdi], 0")?; // Null-terminate the string

    writeln!(file, "convert_loop:")?;
    writeln!(file, "    dec rdi")?; // Move buffer pointer backwards
    writeln!(file, "    div rbx")?; // Divide rax by 10
    writeln!(file, "    add dl, '0'")?; // Convert remainder to ASCII
    writeln!(file, "    mov [rdi], dl")?; // Store ASCII character
    writeln!(file, "    test rax, rax")?; // Check if quotient is zero
    writeln!(file, "    jnz convert_loop")?; // If not zero, convert next digit

    writeln!(file, "    mov rsi, rdi")?; // Set rsi to start of the string
    writeln!(file, "    mov rdx, buffer + 19 - rdi")?; // Set rdx to the length of the string
    writeln!(file, "    ret")?;

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
