use crate::vm;

fn parse_opcode(opcode: &str) -> Result<vm::OpCode, String> {
    match opcode {
        "push" => Ok(vm::OpCode::Push),
        "pop" => Ok(vm::OpCode::Pop),
        "add" => Ok(vm::OpCode::Add),
        "sub" => Ok(vm::OpCode::Sub),
        "neg" => Ok(vm::OpCode::Neg),
        "eq" => Ok(vm::OpCode::Eq),
        "lt" => Ok(vm::OpCode::Lt),
        "le" => Ok(vm::OpCode::Le),
        "gt" => Ok(vm::OpCode::Gt),
        "ge" => Ok(vm::OpCode::Ge),
        "and" => Ok(vm::OpCode::And),
        "or" => Ok(vm::OpCode::Or),
        "not" => Ok(vm::OpCode::Not),
        _ => Err(format!("invalid opcode: {}", opcode)),
    }
}

pub fn generate(command: &str) -> Result<Vec<String>, String> {
    let mut args = command.split(' ');
    let first_arg = args.next().ok_or(String::from("missing opcode arg"))?;
    let opcode = parse_opcode(first_arg)?;

    let remaining_args: Vec<&str> = args.collect();

    match opcode {
        vm::OpCode::Push => vm::push(remaining_args),
        vm::OpCode::Pop => Ok(Vec::with_capacity(1)),
        vm::OpCode::Add => {
            assert!(remaining_args.len() == 0);
            vm::add()
        },
        vm::OpCode::Sub => {
            assert!(remaining_args.len() == 0);
            vm::sub()
        },
        vm::OpCode::Neg => {
            assert!(remaining_args.len() == 0);
            vm::neg()
        },
        vm::OpCode::Eq => {
            assert!(remaining_args.len() == 0);
            vm::eq()
        },
        vm::OpCode::Lt => {
            assert!(remaining_args.len() == 0);
            vm::lt()
        },
        vm::OpCode::Le => {
            assert!(remaining_args.len() == 0);
            vm::le()
        },
        vm::OpCode::Gt => {
            assert!(remaining_args.len() == 0);
            vm::gt()
        },
        vm::OpCode::Ge => {
            assert!(remaining_args.len() == 0);
            vm::ge()
        },
        vm::OpCode::And => {
            assert!(remaining_args.len() == 0);
            vm::and()
        },
        vm::OpCode::Or => {
            assert!(remaining_args.len() == 0);
            vm::or()
        },
        vm::OpCode::Not => {
            assert!(remaining_args.len() == 0);
            vm::not()
        },
    }
}

