use std::collections::HashMap;

const USER_MEM_START: u32 = 16;
const PREDEFINED_SYMBOLS: [(&str, u32); 23] = [
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("SCREEN", 16384),
    ("KBD", 24576),
];

pub fn translate_file(file: &Vec<String>) -> String {
    let (instructions, mut symbols, label_count) = process_raw_file(file);

    let result: Vec<String> = instructions
        .iter()
        .map(|x| translate_line(x, &mut symbols, label_count))
        .collect();

    result.join("\n") + "\n"
}

fn process_raw_file(file: &Vec<String>) -> (Vec<String>, HashMap<&str, u32>, u32) {
    let mut instructions = Vec::with_capacity(file.len());
    let mut labels: HashMap<&str, u32> = PREDEFINED_SYMBOLS.iter().copied().collect();
    let mut inst_count = 0;
    let mut label_count = 0;

    for line in file {
        let clean_line = if let Some((pre_comment, _)) = line.split_once("//") {
            pre_comment.trim()
        } else {
            line.trim()
        };

        if clean_line.starts_with('(') {
            let label = &clean_line[1..clean_line.len() - 1];
            let prev_label = labels.insert(label, inst_count);

            assert_eq!(prev_label, None, "Duplicate labels: {}", label); // TODO: Propogate error upwards
            label_count += 1;
        } else if !clean_line.is_empty() {
            instructions.push(clean_line.to_owned());
            inst_count += 1;
        }
    }

    (instructions, labels, label_count)
}

fn translate_line<'a>(
    line: &'a str,
    symbols: &mut HashMap<&'a str, u32>,
    label_count: u32,
) -> String {
    let first_char = line.chars().next();

    match first_char {
        Some('@') => translate_a(line, symbols, label_count),
        _ => translate_c(line),
    }
}

fn translate_a<'a>(
    line: &'a str,
    symbols: &mut HashMap<&'a str, u32>,
    symbol_count: u32,
) -> String {
    let symbol = &line[1..];

    if let Ok(address) = symbol.parse::<u16>() {
        format!("{:016b}", address)
    } else if let Some(address) = symbols.get(symbol) {
        return format!("{:016b}", address);
    } else {
        let address = next_free_mem_addr(symbols, symbol_count);
        symbols.insert(symbol, address);

        return format!("{:016b}", address);
    }
}

fn next_free_mem_addr(symbols: &HashMap<&str, u32>, label_count: u32) -> u32 {
    let address = symbols.len() as u32;
    let address = address + USER_MEM_START;
    let address = address - PREDEFINED_SYMBOLS.len() as u32;
    

    address - label_count
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

    format!("111{}{}{}", comp, dest, jump)
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

    format!("{:03b}", result)
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
