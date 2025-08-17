use std::io::Write;

use shared::hack;

use crate::opcode::{LabelCounter, OpCode};
use crate::parser::VmFile;

pub(crate) struct Writer {
    input: Vec<VmFile>,
    output: std::io::Stdout,
    label_counter: LabelCounter,
}

impl Writer {
    pub(crate) fn new(files: Vec<VmFile>) -> Self {
        let output = std::io::stdout();

        // Check if we can/need to generate the bootstrap code.
        let mut label_counter = LabelCounter::default();
        if files
            .iter()
            .flat_map(|file| file.opcodes.iter())
            .any(|(_, _, opcode)| match opcode {
                Ok(OpCode::Function { name, .. }) => name == "Sys.init",
                _ => false,
            })
        {
            let mut lock = output.lock();
            for ix in Self::bootstrap_code(&mut label_counter) {
                writeln!(&mut lock, "{ix}").unwrap();
            }
        }

        Writer { input: files, output, label_counter }
    }

    pub(crate) fn write(mut self) {
        let mut lock = self.output.lock();
        for (_, (line, source, res)) in self
            .input
            .iter()
            .flat_map(|file| file.opcodes.iter().map(|opcode| (&file.path, opcode)))
        {
            let opcode = match res {
                Ok(opcode) => opcode,
                Err(err) => {
                    writeln!(&mut lock, "ERR: {err}").unwrap();
                    continue;
                }
            };

            writeln!(&mut lock, "// L{line}: {source}").unwrap();
            for ix in opcode.bytecode(&mut self.label_counter) {
                writeln!(&mut lock, "{ix}").unwrap();
            }
        }
    }

    fn bootstrap_code(label_counter: &mut LabelCounter) -> impl Iterator<Item = hack::Instruction> {
        [hack!("@256"), hack!("D=A"), hack!("@SP"), hack!("M=D")]
            .into_iter()
            .chain(OpCode::Call { name: "Sys.init".to_string(), args: 0 }.bytecode(label_counter))
    }
}
