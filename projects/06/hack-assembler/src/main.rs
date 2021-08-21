use std::env;
use std::error::Error;
use std::fs;

fn read_lines(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;
    let lines = file.lines().map(str::to_owned).collect();

    return Ok(lines);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        let result = read_lines(&first_arg).unwrap();

        for (i, line) in result.iter().enumerate() {
            println!("{}: {}", i, line);
        }
    }
}
