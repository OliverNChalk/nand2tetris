use std::str::FromStr;

use thiserror::Error;

use super::{AluOutput, Assignment, Branch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    A(u16),
    C(Assignment, AluOutput, Branch),
}

impl FromStr for Instruction {
    type Err = ParseInstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let to_err = |_| ParseInstructionErr(s.to_owned());

        if s.starts_with('@') {
            unimplemented!()
        } else {
            let has_assignment = s.contains('=');
            let has_branch = s.contains(';');

            match (has_assignment, has_branch) {
                (true, true) => {
                    let (assignment, rest) = s.split_once('=').unwrap();
                    let (operation, branch) = rest.split_once(';').unwrap();

                    Ok(Instruction::C(
                        assignment.parse().map_err(to_err)?,
                        operation.parse().map_err(to_err)?,
                        branch.parse().map_err(to_err)?,
                    ))
                }
                (true, false) => {
                    let (assignment, operation) = s.split_once('=').unwrap();

                    Ok(Instruction::C(
                        assignment.parse().map_err(to_err)?,
                        operation.parse().map_err(to_err)?,
                        Branch::None,
                    ))
                }
                (false, true) => {
                    let (operation, branch) = s.split_once('=').unwrap();

                    Ok(Instruction::C(
                        Assignment::None,
                        operation.parse().map_err(to_err)?,
                        branch.parse().map_err(to_err)?,
                    ))
                }
                (false, false) => Err(ParseInstructionErr(s.to_owned())),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Failed to parse instruction; instruction={0}")]
pub struct ParseInstructionErr(pub String);
