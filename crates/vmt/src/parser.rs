use std::path::Path;

use crate::opcode::{OpCode, ParseOpCodeErr};
use crate::region::Region;

pub(crate) struct VmFile {
    pub(crate) opcodes: Vec<(usize, String, Result<OpCode, ParseOpCodeErr>)>,
    pub(crate) static_variables: u16,
}

impl VmFile {
    pub(crate) fn parse_file(path: &Path) -> VmFile {
        let opcodes: Vec<_> = std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| {
                line.split_once("//")
                    .map(|(left, _)| left)
                    .unwrap_or(line)
                    .trim()
            })
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(number, source)| (number + 1, source.to_owned(), source.parse::<OpCode>()))
            .collect();
        let static_variables = opcodes
            .iter()
            .filter_map(|(_, _, opcode)| match opcode {
                Ok(OpCode::Push(Region::Static, offset) | OpCode::Pop(Region::Static, offset)) => {
                    Some(offset)
                }
                _ => None,
            })
            .max()
            .map_or(0, |offset| offset + 1);

        VmFile { opcodes, static_variables }
    }
}
