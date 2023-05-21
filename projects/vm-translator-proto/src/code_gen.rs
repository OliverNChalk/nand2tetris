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

fn parse_region(region: &str) -> Result<vm::Region, String> {
    match region {

        "constant" => Ok(vm::Region::Constant),
        "pointer" => Ok(vm::Region::Pointer),
        "temp" => Ok(vm::Region::Temp),
        "static" => Ok(vm::Region::Temp),
        "local" => Ok(vm::Region::Local),
        "argument" => Ok(vm::Region::Argument),
        "this" => Ok(vm::Region::This),
        "that" => Ok(vm::Region::That),
        _ => Err(format!("invalid region: {}", region)),
    }
}

pub fn generate(command: &str, label_count: &mut vm::Counter) -> Result<Vec<String>, String> {
    let mut args = command.split(' ');
    let first_arg = args.next().ok_or(String::from("missing opcode arg"))?;
    let opcode = parse_opcode(first_arg)?;

    let remaining_args: Vec<&str> = args.collect();

    match opcode {
        vm::OpCode::Push => {
            let region = parse_region(remaining_args.get(0).unwrap())?;
            let index: u32 = remaining_args.get(1).unwrap().parse().unwrap();

            Ok(vm::push(region, index))
        },
        vm::OpCode::Pop => {
            let region = parse_region(remaining_args.get(0).unwrap())?;
            let index: u32 = remaining_args.get(1).unwrap().parse().unwrap();

            Ok(vm::pop(region, index))
        },
        vm::OpCode::Add => {
            assert!(remaining_args.len() == 0);
            Ok(vm::add())
        },
        vm::OpCode::Sub => {
            assert!(remaining_args.len() == 0);
            Ok(vm::sub())
        },
        vm::OpCode::Neg => {
            assert!(remaining_args.len() == 0);
            Ok(vm::neg())
        },
        vm::OpCode::Eq => {
            assert!(remaining_args.len() == 0);
            Ok(vm::eq(label_count))
        },
        vm::OpCode::Lt => {
            assert!(remaining_args.len() == 0);
            Ok(vm::lt(label_count))
        },
        vm::OpCode::Le => {
            assert!(remaining_args.len() == 0);
            Ok(vm::le(label_count))
        },
        vm::OpCode::Gt => {
            assert!(remaining_args.len() == 0);
            Ok(vm::gt(label_count))
        },
        vm::OpCode::Ge => {
            assert!(remaining_args.len() == 0);
            Ok(vm::ge(label_count))
        },
        vm::OpCode::And => {
            assert!(remaining_args.len() == 0);
            Ok(vm::and())
        },
        vm::OpCode::Or => {
            assert!(remaining_args.len() == 0);
            Ok(vm::or())
        },
        vm::OpCode::Not => {
            assert!(remaining_args.len() == 0);
            Ok(vm::not())
        },
    }
}

