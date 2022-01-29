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

pub fn generate(command: &str) -> Result<Vec<&str>, String> {
    let mut args = command.split(' ');
    let first_arg = args.next().ok_or(format!("missing opcode arg"))?;
    let opcode = parse_opcode(first_arg)?;

    let remaining_args: Vec<&str> = args.collect();

    match opcode {
        VmOpCodes::Push => vm_push(remaining_args),
        _ => Ok(vec!["UNIMPLEMENTED", first_arg]),
    }
}

// opcode implementations
fn vm_push(args: Vec<&str>) -> Result<Vec<&str>, String> {
    Ok(vec!["coming", "soon"])
}
