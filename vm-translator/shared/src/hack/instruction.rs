use std::fmt::Display;
use std::str::FromStr;

use super::{AluOutput, Assignment, Branch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    A(u16),
    C(Option<Assignment>, AluOutput, Option<Branch>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::A(address) => write!(f, "@{address}"),
            Instruction::C(assignment, alu_output, branch) => {
                if let Some(assignment) = assignment {
                    write!(f, "{assignment}=")?;
                }

                write!(f, "{alu_output}")?;

                if let Some(branch) = branch {
                    write!(f, ";{branch}")?;
                }

                Ok(())
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('@') {
            unimplemented!()
        } else {
            let mut alu_output = None;

            // Parse assignment if it exists (setting alu ix at the same time).
            let assignment = match s.split_once('=') {
                Some((assignment, rest)) => {
                    alu_output = Some(rest.split(';').next().unwrap_or(rest).parse()?);
                    Some(assignment.parse()?)
                }
                None => None,
            };

            // Parse branch if it exists (setting alu ix at the same time).
            let branch = match s.split_once(';') {
                Some((rest, branch)) => {
                    alu_output = Some(rest.split('=').nth(1).unwrap_or(rest).parse()?);
                    Some(branch.parse()?)
                }
                None => None,
            };

            alu_output
                .ok_or(strum::ParseError::VariantNotFound)
                .map(|alu_output| Instruction::C(assignment, alu_output, branch))
        }
    }
}
