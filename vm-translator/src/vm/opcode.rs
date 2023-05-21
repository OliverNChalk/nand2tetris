use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub(crate) enum OpCode {
    // Memory access
    Push(super::Region, u32),
    Pop(super::Region, u32),

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
