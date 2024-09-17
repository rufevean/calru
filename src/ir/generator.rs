
use crate::ast::{AST, ASTNode};
use crate::ir::instruction::IRInstruction;

pub fn generate_ir(ast: &AST) -> Vec<IRInstruction> {
    let mut instructions = Vec::new();
    generate_ir_node(ast, &mut instructions);
    instructions
}


fn generate_ir_node(node: &AST, instructions: &mut Vec<IRInstruction>) {
    match &node.node {
        ASTNode::Int(value) => {
            instructions.push(IRInstruction::Mov { dest: "R0".to_string(), src: value.to_string() });
        }
        ASTNode::Float(value) => {
            instructions.push(IRInstruction::Mov { dest: "R0".to_string(), src: value.to_string() });
        }
        ASTNode::Identifier(var) => {
            instructions.push(IRInstruction::Mov { dest: "R0".to_string(), src: var.clone() });
        }
        ASTNode::BinaryOperation { operator, left, right } => {
            generate_ir_node(left, instructions);
            generate_ir_node(right, instructions);
            match operator.as_str() {
                "+" => instructions.push(IRInstruction::Add { dest: "R0".to_string(), src: "R1".to_string() }),
                "-" => instructions.push(IRInstruction::Sub { dest: "R0".to_string(), src: "R1".to_string() }),
                "*" => instructions.push(IRInstruction::Mul { dest: "R0".to_string(), src: "R1".to_string() }),
                "/" => instructions.push(IRInstruction::Div { dest: "R0".to_string(), src: "R1".to_string() }),
                _ => panic!("Unsupported operator"),
            }
        }
        ASTNode::Assignment { variable, expression } => {
            generate_ir_node(expression, instructions);
            instructions.push(IRInstruction::Mov { dest: variable.clone(), src: "R0".to_string() });
        }
        ASTNode::Print(operand) => {
            generate_ir_node(operand, instructions);
            instructions.push(IRInstruction::Print { operand: "R0".to_string() });
        }
        _ => {
            println!("Unsupported AST node: {:?}", node.node); 
        }
    }
}
