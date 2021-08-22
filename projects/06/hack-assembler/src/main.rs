// use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn read_lines(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;
    let lines = file.lines().map(str::to_owned).collect();

    return Ok(lines);
}

fn clean_whitespace(mut file: Vec<String>) -> Vec<String> {
    for i in 0..file.len() {
        let line = &mut file[i];

        // Remove everything after a comment
        if let Some(test) = line.find("//") {
            line.truncate(test);
        }

        // Remove all whitespace
        *line = line.trim().to_owned();
    }

    return file;
}

// fn translate_line(line: &str) -> String {
//
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        let result = read_lines(&first_arg).unwrap();
        let result = clean_whitespace(result);

        for (i, line) in result.iter().enumerate() {
            println!("{}: {}", i, line);
        }
    }
}
