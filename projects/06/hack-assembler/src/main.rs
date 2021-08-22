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

fn translate_line(line: &str) -> String {
    let first_char = line.chars().nth(0);

    return match first_char {
        None => "EMPTY LINE".to_owned(),
        Some('(') => "SYMBOL".to_owned(),
        Some('@') => translate_a(line).unwrap_or("".to_owned()),
        _ => translate_c(line),
    };
}

fn translate_a(line: &str) -> Option<String> {
    let symbol = &line[1..];

    let result = symbol
        .parse::<u16>()
        .and_then(|x| Ok(format!("{:016b}", x)))
        .expect(&format!("Failed to parse to A-INST {}", symbol));

    let lead_bit = result.chars().nth(0).unwrap();
    assert_eq!(lead_bit, '0', "{} lead bit was not zero", lead_bit);

    return Some(result);
}

fn translate_c(line: &str) -> String {
    if let Some((dest, _)) = line.split_once('=') {
        return format!("DEST: {}", dest);
    }

    if let Some((_, jump)) = line.split_once(';') {
        return format!("JUMP: {}", jump);
    } else {
        return format!("INVALID C-INST");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        let result = read_lines(&first_arg).unwrap();
        let result = clean_whitespace(result);

        for (i, line) in result.iter().enumerate() {
            println!("{}: {}", i, line);
            println!("{}", translate_line(line));
        }
    }
}
