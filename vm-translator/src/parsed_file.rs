use std::path::Path;

use crate::vm;

#[derive(Debug)]
pub(crate) struct ParsedFile {
    source: Vec<(usize, Result<vm::OpCode, vm::ParseOpCodeErr>)>,
}

impl ParsedFile {
    pub(crate) fn from_file(file: &Path) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(&file)?;

        let source = raw
            .lines()
            .map(|line| line.trim())
            .enumerate()
            .filter(|(_, line)| !(line.starts_with("//") || line.len() == 0))
            .map(|(number, line)| (number + 1, line.parse()))
            .collect();

        Ok(ParsedFile { source })
    }
}
