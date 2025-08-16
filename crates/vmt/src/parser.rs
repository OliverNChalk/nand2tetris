use std::path::PathBuf;

use crate::opcode::{OpCode, ParseOpCodeErr};

pub(crate) struct VmFile {
    pub(crate) path: PathBuf,
    pub(crate) opcodes: Vec<(usize, String, Result<OpCode, ParseOpCodeErr>)>,
}

impl VmFile {
    pub(crate) fn parse_file(path: PathBuf) -> VmFile {
        let opcodes = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|line| line.trim())
            .enumerate()
            .filter(|(_, line)| !line.is_empty() && !line.starts_with("//"))
            .map(|(number, source)| (number + 1, source.to_owned(), source.parse::<OpCode>()))
            .collect();

        VmFile { path, opcodes }
    }
}
