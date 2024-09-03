use std::str::FromStr;

use eyre::eyre;
use shared::hack;

use super::OpCode;

pub(crate) enum Element {
    Opcode(OpCode),
    Label(String),
}

impl Element {
    pub(crate) fn bytecode(&self, labels: &mut hack::Labels) -> Vec<hack::Instruction> {
        match self {
            Element::Opcode(opcode) => opcode.bytecode(labels),
            Element::Label(label) => vec![hack::Instruction::Label(label.to_owned())],
        }
    }
}

impl FromStr for Element {
    type Err = eyre::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        // Early return if not a label.
        if !source.starts_with("label") {
            return Ok(Element::Opcode(source.parse()?));
        }

        // Handle labels inline.
        let mut words = source.split(' ');
        assert_eq!(words.next().unwrap(), "label");

        // Next word is the label.
        let label = words
            .next()
            .ok_or_else(|| eyre!("Missing label; source={source}"))?;

        // There should be no additional words.
        if words.next().is_some() {
            return Err(eyre!("Unexpected trailing word; source={source}"));
        }

        Ok(Element::Label(label.to_owned()))
    }
}
