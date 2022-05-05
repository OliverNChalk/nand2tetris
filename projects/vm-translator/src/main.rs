use std::{env, fs, io, path};

use vm_translator::{code_gen, parser, vm};

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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "invalid number of arguments");

    let argument = args.get(1).unwrap();

    let source = path::Path::new(argument).canonicalize().map_err(|e| e.to_string())?;
    let result = handle_file(source.to_str().unwrap())?;

    // extract dir
    let file_stem = source.file_stem().unwrap().to_str().unwrap();
    let file_dir = source.ancestors().nth(1).unwrap().to_str().unwrap();

    let result_file = format!("{}/{}.asm", file_dir, file_stem);

    println!("{}", result_file);

    write_result(&result_file, result).map_err(|e| e.to_string())
}
