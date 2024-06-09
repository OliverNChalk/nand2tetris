use std::str::FromStr;

use shared::{assign, hack};
use thiserror::Error;

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
    fn write_head() -> [hack::Instruction; 3] {
        [hack::Instruction::A(0), assign!("A=M"), assign!("M=D")]
    }

    fn increment_stack() -> [hack::Instruction; 2] {
        [hack::Instruction::A(0), assign!("M=M+1")]
    }

    pub(crate) fn bytecode(&self) -> Vec<hack::Instruction> {
        match self {
            OpCode::Push(region, index) => match region.offset() {
                super::RegionType::Constant => [hack::Instruction::A(*index), assign!("D=A")]
                    .into_iter()
                    .chain(Self::write_head())
                    .chain(Self::increment_stack())
                    .collect(),
                super::RegionType::Fixed(offset) => {
                    [hack::Instruction::A(offset + index), assign!("D=M")]
                        .into_iter()
                        .chain(Self::write_head())
                        .chain(Self::increment_stack())
                        .collect()
                }

                super::RegionType::Dynamic(offset) => [
                    hack::Instruction::A(offset),
                    assign!("D=M"),
                    hack::Instruction::A(*index),
                    assign!("D=D+A"),
                    assign!("A=D"),
                    assign!("D=M"),
                ]
                .into_iter()
                .chain(Self::write_head())
                .chain(Self::increment_stack())
                .collect(),
            },
            _ => todo!(),
        }
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
