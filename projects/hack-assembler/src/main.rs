use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        let (file_name, _) = first_arg.split_once('.').unwrap();
        let lines = read_lines(&first_arg).unwrap_or_else(|e| {
            eprintln!("Could not read file ({}): {}", first_arg, e);
            process::exit(1);
        });

        let result = hack_assembler::translate_file(&lines);
        fs::write(format!("{}.hack", file_name), result).unwrap_or_else(|e| {
            eprintln!("Could not write file {}.hack: {}", file_name, e);
        });
    }
}

fn read_lines(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;
    let lines = file.lines().map(str::to_owned).collect();

    return Ok(lines);
}
