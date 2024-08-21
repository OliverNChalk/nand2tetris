use std::str::FromStr;

use shared::hack;
use thiserror::Error;

use super::RegionType;

#[derive(Debug)]
pub(crate) enum OpCode {
    // Memory access
    Push(super::Region, u16),
    Pop(super::Region, u16),

    // Arithmetic
    Add,
    Sub,
    Neg,

    // Comparison
    Eq,
    Lt,
    Le,
    Gt,
    Ge,

    // Bitwise logical
    And,
    Or,
    Not,
}

impl OpCode {
    pub(crate) fn bytecode(&self) -> Vec<hack::Instruction> {
        match self {
            OpCode::Push(region, index) => match region.offset() {
                RegionType::Constant => [hack::Instruction::A(*index), hack!("D=A")]
                    .into_iter()
                    .chain(Self::write_head())
                    .chain(Self::increment_stack())
                    .collect(),
                RegionType::Fixed(offset) => [hack::Instruction::A(offset + index), hack!("D=M")]
                    .into_iter()
                    .chain(Self::write_head())
                    .chain(Self::increment_stack())
                    .collect(),

                RegionType::Dynamic(offset) => [
                    hack!("@{offset}"),
                    hack!("D=M"),
                    hack!("@{index}"),
                    hack!("D=D+A"),
                    hack!("A=D"),
                    hack!("D=M"),
                ]
                .into_iter()
                .chain(Self::write_head())
                .chain(Self::increment_stack())
                .collect(),
            },
            OpCode::Pop(region, index) => match region.offset() {
                RegionType::Constant => panic!("Cannot pop to constant"),
                RegionType::Dynamic(offset) => Self::decrement_stack()
                    .into_iter()
                    .chain(Self::read_head())
                    // TODO: This seems suboptimal, was lifted from old impl.
                    .chain([
                        // Store HEAD in R13.
                        hack!("@R13"),
                        hack!("M=D"),
                        // Load offset.
                        hack!("@{offset}"),
                        hack!("D=M"),
                        // Add index to offset.
                        hack!("@{index}"),
                        hack!("D=D+A"),
                        // Store address in R14.
                        hack!("@R14"),
                        hack!("M=D"),
                        // Load HEAD into D.
                        hack!("@R13"),
                        hack!("D=M"),
                        // Load address into A.
                        hack!("@R14"),
                        hack!("A=M"),
                        // Write D to address.
                        hack!("M=D"),
                    ])
                    .collect(),
                RegionType::Fixed(offset) => Self::decrement_stack()
                    .into_iter()
                    .chain(Self::read_head())
                    .chain([hack!("@{}", offset + index), hack!("M=D")])
                    .collect(),
            },
            OpCode::Add => Self::decrement_stack()
                .into_iter()
                .chain(Self::read_head())
                .chain(Self::decrement_stack())
                .chain([hack!("A=M"), hack!("D=D+M")])
                .chain(Self::write_head())
                .chain(Self::increment_stack())
                .collect(),
            OpCode::Sub => Self::decrement_stack()
                .into_iter()
                .chain(Self::read_negated_head())
                .chain(Self::decrement_stack())
                .chain([hack!("A=M"), hack!("D=D+M")])
                .chain(Self::write_head())
                .chain(Self::increment_stack())
                .collect(),
            opcode => todo!("opcode={opcode:?}"),
        }
    }

    fn read_head() -> [hack::Instruction; 2] {
        [hack!("A=M"), hack!("D=M")]
    }

    fn read_negated_head() -> [hack::Instruction; 2] {
        [hack!("A=M"), hack!("D=-M")]
    }

    fn write_head() -> [hack::Instruction; 3] {
        [hack::Instruction::A(0), hack!("A=M"), hack!("M=D")]
    }

    fn increment_stack() -> [hack::Instruction; 2] {
        [hack::Instruction::A(0), hack!("M=M+1")]
    }

    fn decrement_stack() -> [hack::Instruction; 2] {
        [hack::Instruction::A(0), hack!("M=M-1")]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub(crate) enum ParseOpCodeErr {
    #[error("Invalid opcode; line={0}")]
    Opcode(String),
    #[error("Invalid argument count; line={0}")]
    ArgumentCount(String),
    #[error("Invalid region; line={0}")]
    Region(super::ParseRegionErr),
    #[error("Invalid index; line={0}")]
    Index(std::num::ParseIntError),
}

impl FromStr for OpCode {
    type Err = ParseOpCodeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.trim().split(' ');
        let first = words.next().ok_or_else(|| todo!())?;

        match first {
            "push" => {
                let region = words
                    .next()
                    .ok_or_else(|| ParseOpCodeErr::ArgumentCount(s.to_owned()))?
                    .parse()
                    .map_err(ParseOpCodeErr::Region)?;
                let index = words
                    .next()
                    .ok_or_else(|| ParseOpCodeErr::ArgumentCount(s.to_owned()))?
                    .parse()
                    .map_err(ParseOpCodeErr::Index)?;

                Ok(OpCode::Push(region, index))
            }
            "pop" => {
                let region = words
                    .next()
                    .ok_or_else(|| ParseOpCodeErr::ArgumentCount(s.to_owned()))?
                    .parse()
                    .map_err(ParseOpCodeErr::Region)?;
                let index = words
                    .next()
                    .ok_or_else(|| ParseOpCodeErr::ArgumentCount(s.to_owned()))?
                    .parse()
                    .map_err(ParseOpCodeErr::Index)?;

                Ok(OpCode::Pop(region, index))
            }
            "add" => Ok(OpCode::Add),
            "sub" => Ok(OpCode::Sub),
            "neg" => Ok(OpCode::Neg),
            "eq" => Ok(OpCode::Eq),
            "lt" => Ok(OpCode::Lt),
            "le" => Ok(OpCode::Le),
            "gt" => Ok(OpCode::Gt),
            "ge" => Ok(OpCode::Ge),
            "and" => Ok(OpCode::And),
            "or" => Ok(OpCode::Or),
            "not" => Ok(OpCode::Not),
            _ => Err(ParseOpCodeErr::Opcode(s.to_owned())),
        }
    }
}
