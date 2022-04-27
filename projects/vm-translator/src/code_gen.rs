use std::fmt;

enum VmOpCodes {
    // memory access
    Push,
    Pop,
    // arithmetic
    Add,
    Sub,
    Neg,
    // comparison
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    // bitwise logical
    And,
    Or,
    Not,
}

fn parse_opcode(opcode: &str) -> Result<VmOpCodes, String> {
    match opcode {
        "push" => Ok(VmOpCodes::Push),
        "pop" => Ok(VmOpCodes::Pop),
        "add" => Ok(VmOpCodes::Add),
        "sub" => Ok(VmOpCodes::Sub),
        "neg" => Ok(VmOpCodes::Neg),
        "eq" => Ok(VmOpCodes::Eq),
        "lt" => Ok(VmOpCodes::Lt),
        "le" => Ok(VmOpCodes::Le),
        "gt" => Ok(VmOpCodes::Gt),
        "ge" => Ok(VmOpCodes::Ge),
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
        VmOpCodes::Add => {
            assert!(remaining_args.len() == 0);
            vm_add()
        },
        _ => Ok(vec![String::from("UNIMPLEMENTED"), String::from(first_arg)]),
    }
}

// hack helpers
// opcode implementations
fn vm_push(args: Vec<&str>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err(String::from("invalid push arguments"));
    }

    let region = args.get(0).ok_or(String::from("push: missing region"))?;
    assert_eq!(*region, "constant");

    let value = args.get(1).ok_or(String::from("push: missing value"))?;

    Ok(vec![
        // store value in D
        format!("@{}", value),
        format!("D=A"),
        // push D to the top of the stack
        format!("@0"),
        format!("A=M"),
        format!("M=D"),
        // point stack at next free slot
        format!("@0"),
        format!("M=M+1"),
    ])
}

fn vm_add() -> Result<Vec<String>, String> {
    // 1. pops the top 2 stack elements
    // 2. adds the popped elements
    // 3. pushes the result onto the stack
    Ok(vec![
        // todo: can we assume all operations set A = SP?
        // read the first data value at SP-1
        format!("@0"),
        format!("M=M-1"),
        // load the first value from the stack
        format!("A=M"),
        format!("D=M"),
        // decrement the stack pointer to read next value
        format!("@0"),
        format!("M=M-1"),
        // add the next value to D register
        format!("A=M"),
        format!("D=D+M"),
        // overwrite the previous value with D
        format!("@0"),
        format!("A=M"),
        format!("M=D"),
        // point stack at next free slot
        format!("@0"),
        format!("M=M+1"),
    ])
}

