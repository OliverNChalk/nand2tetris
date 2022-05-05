use std::{env, fs, io, path};

use vm_translator::{code_gen, parser, vm, optimizer};

fn handle_file(file_path: &str) -> Result<Vec<String>, String> {
    let file = fs::read_to_string(file_path)
        .map_err(|x| format!("failed to read '{}' with error: {}", file_path, x))?;

    let jack_file = parser::JackFile::new(&file);
    let mut result = vec![];
    let mut errors = vec![];

    let mut label_count = vm::Counter::new();

    for (line_number, command) in jack_file.commands() {
        let hack_assembly = code_gen::generate(command, &mut label_count);

        match hack_assembly {
            Ok(mut hack_assembly) => {
                result.push(format!("// L{}: {}", line_number, command));
                result.append(&mut hack_assembly);
                result.push("".to_owned());
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    if errors.len() == 0 {
        Ok(result)
    } else {
        errors.iter().for_each(|e| println!("{}", e));

        Err(format!("parsing failed with {} errors", errors.len()))
    }
}

fn write_result(file_path: &str, result: Vec<String>) -> io::Result<()> {
    let content = result.join("\n");

    fs::write(file_path, content)
}

fn parse_arguments<'a>(args: &'a Vec<String>) -> Result<(&'a path::Path, bool), String> {
    if args.len() < 2 || args.len() > 3 {
        return Err(format!(
            "invalid number of arguments, received: {}, expected: {}",
            args.len(),
            "2-3",
        ))
    }

    let mut path = None;
    let mut should_optimize = false;
    if let Some(arg1) = args.get(1) {
        if *arg1 == "--optimize".to_owned() {
            should_optimize = true;
        } else {
            path = Some(path::Path::new(arg1));
        }
    }
    if let Some(arg2) = args.get(2) {
        if *arg2 == "--optimize".to_owned() {
            should_optimize = true;
        } else {
            path = Some(path::Path::new(arg2));
        }
    }

    let path = path.ok_or("failed to parse path from arguments".to_owned())?;

    Ok((path, should_optimize))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let (source, should_optimize) = parse_arguments(&args)?;

    let result = handle_file(source.to_str().unwrap())?;
    // optimize if flag was set
    let result = if should_optimize {
        optimizer::optimize(result)
    } else {
        result
    };

    // extract dir
    let file_stem = source.file_stem().unwrap().to_str().unwrap();
    let file_dir = source.ancestors().nth(1).unwrap().to_str().unwrap();

    let result_file = format!("{}/{}.asm", file_dir, file_stem);

    write_result(&result_file, result).map_err(|e| e.to_string())
}
