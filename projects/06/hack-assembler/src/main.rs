// use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        let result = read_lines(&first_arg).unwrap();
        let result = clean_whitespace(result);

        for line in result.iter() {
            println!("{}", translate_line(line));
        }
    }
}

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
        None => "".to_owned(),
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
    let boundaries = &['=', ';'][..];
    line.split(boundaries).next();

    let mut dest = None;
    let mut comp = line;
    let mut jump = None;
    if let Some(split) = line.split_once('=') {
        dest = Some(split.0);
        comp = split.1;
    }

    if let Some(split) = line.split_once(';') {
        comp = split.0;
        jump = Some(split.1);
    }

    let dest = translate_dest(dest);
    let comp = translate_comp(comp);
    let jump = translate_jump(jump);

    return format!("111{}{}{}", comp, dest, jump);
}

fn translate_dest(source: Option<&str>) -> String {
    let mut result = 0;

    if let Some(dest) = source {
        if dest.contains('A') {
            result += 4;
        }
        if dest.contains('D') {
            result += 2;
        }
        if dest.contains('M') {
            result += 1;
        }
    }

    return format!("{:03b}", result);
}

fn translate_comp(source: &str) -> String {
    match source {
        "0" => "0101010".to_owned(),
        "1" => "0111111".to_owned(),
        "-1" => "0111010".to_owned(),
        "D" => "0001100".to_owned(),
        "A" => "0110000".to_owned(),
        "!D" => "0001101".to_owned(),
        "!A" => "0110001".to_owned(),
        "-D" => "0001111".to_owned(),
        "-A" => "0110011".to_owned(),
        "D+1" => "0011111".to_owned(),
        "A+1" => "0110111".to_owned(),
        "D-1" => "0001110".to_owned(),
        "A-1" => "0110010".to_owned(),
        "D+A" => "0000010".to_owned(),
        "D-A" => "0010011".to_owned(),
        "A-D" => "0000111".to_owned(),
        "D&A" => "0000000".to_owned(),
        "D|A" => "0010101".to_owned(),
        "M" => "1110000".to_owned(),
        "!M" => "1110001".to_owned(),
        "-M" => "1110011".to_owned(),
        "M+1" => "1110111".to_owned(),
        "M-1" => "1110010".to_owned(),
        "D+M" => "1000010".to_owned(),
        "D-M" => "1010011".to_owned(),
        "M-D" => "1000111".to_owned(),
        "D&M" => "1000000".to_owned(),
        "D|M" => "1010101".to_owned(),
        _ => panic!("Invalid comp instruction: {}", source),
    }
}

fn translate_jump(source: Option<&str>) -> String {
    if let Some(jump) = source {
        match jump {
            "JGT" => "001".to_owned(),
            "JEQ" => "010".to_owned(),
            "JGE" => "011".to_owned(),
            "JLT" => "100".to_owned(),
            "JNE" => "101".to_owned(),
            "JLE" => "110".to_owned(),
            "JMP" => "111".to_owned(),
            _ => panic!("Invalid jump instruction: {}", jump),
        }
    } else {
        "000".to_owned()
    }
}
