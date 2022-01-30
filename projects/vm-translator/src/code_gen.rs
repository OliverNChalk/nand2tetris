use std::fmt;

enum VmOpCodes {
    // memory access
    Push,
    Pop,
    // arithmetic
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

const SP: u16 = 0;

fn parse_opcode(opcode: &str) -> Result<VmOpCodes, String> {
    match opcode {
        "push" => Ok(VmOpCodes::Push),
        "pop" => Ok(VmOpCodes::Pop),
        "add" => Ok(VmOpCodes::Add),
        "sub" => Ok(VmOpCodes::Sub),
        "neg" => Ok(VmOpCodes::Neg),
        "eq" => Ok(VmOpCodes::Eq),
        "gt" => Ok(VmOpCodes::Gt),
        "lt" => Ok(VmOpCodes::Lt),
        "and" => Ok(VmOpCodes::And),
        "or" => Ok(VmOpCodes::Or),
        "not" => Ok(VmOpCodes::Not),
        _ => Err(format!("invalid opcode: {}", opcode)),
    }
}

pub fn generate(command: &str) -> Result<Vec<String>, String> {
    let mut args = command.split(' ');
    let first_arg = args.next().ok_or(String::from("missing opcode arg"))?;
    let opcode = parse_opcode(first_arg)?;

    let remaining_args: Vec<&str> = args.collect();

    match opcode {
        VmOpCodes::Push => vm_push(remaining_args),
        _ => Ok(vec![String::from("UNIMPLEMENTED"), String::from(first_arg)]),
    }
}

// hack helpers
fn set_a(val: u16) -> String {
    format!("@{}", val)
}

fn inc_sp() -> Vec<String> {
    vec![set_a(SP), String::from("M=M+1")]
}

// opcode implementations
fn vm_push(args: Vec<&str>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err(String::from("invalid push arguments"));
    }

    let region = args.get(0).ok_or(String::from("push: missing region"))?;
    let value = args.get(1).ok_or(String::from("push: missing value"))?;

    Ok(vec![
        format!("@{}", value),
        String::from("D=A"),
        set_a(SP),
        String::from("A=M"),
        String::from("M=D"),
        set_a(SP),
        String::from("M=M+1"),
    ])
}
