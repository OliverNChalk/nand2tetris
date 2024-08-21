use std::fmt::Display;
use std::str::FromStr;

use eyre::{eyre, OptionExt};

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
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(target) = s.strip_prefix('@') {
            match target.chars().next() {
                Some('R') => {
                    let register = target
                        .get(1..)
                        .ok_or_eyre("Missing register number; instruction={s}")?;
                    let register = register.parse::<u16>()?;
                    eyre::ensure!(
                        register < 16,
                        "Invalid register; register={register}; instruction={s}"
                    );

                    Ok(Instruction::A(register))
                }
                Some('0'..='9') => Ok(Instruction::A(target.parse()?)),
                Some(_) => {
                    let symbol = PredefinedSymbols::from_str(target)?;

                    Ok(Instruction::A(symbol.address()))
                }
                _ => Err(eyre!("Invalid A instruction; instruction={s}")),
            }
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
                .ok_or(eyre!("Invalid C instruction; instruction={s}"))
                .map(|alu_output| Instruction::C(assignment, alu_output, branch))
        }
    }
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum PredefinedSymbols {
    Sp,
    Lcl,
    Arg,
    This,
    That,
    Screen,
    Kbd,
}

impl PredefinedSymbols {
    pub fn address(&self) -> u16 {
        match self {
            Self::Sp => 0,
            Self::Lcl => 1,
            Self::Arg => 2,
            Self::This => 3,
            Self::That => 4,
            Self::Screen => 16384,
            Self::Kbd => 24576,
        }
    }
}
