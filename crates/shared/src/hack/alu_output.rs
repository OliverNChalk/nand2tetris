use strum::EnumString;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, EnumString)]
pub enum AluOutput {
    #[strum(to_string = "0")]
    ZERO,
    #[strum(to_string = "1")]
    ONE,
    #[strum(to_string = "-1")]
    NEGATIVE_ONE,
    #[strum(to_string = "D")]
    D,
    #[strum(to_string = "A")]
    A,
    #[strum(to_string = "M")]
    M,
    #[strum(to_string = "!D")]
    NEGATE_D,
    #[strum(to_string = "!A")]
    NEGATE_A,
    #[strum(to_string = "!M")]
    NEGATE_M,
    #[strum(to_string = "-D")]
    NEGATIVE_D,
    #[strum(to_string = "-A")]
    NEGATIVE_A,
    #[strum(to_string = "-M")]
    NEGATIVE_M,
    #[strum(to_string = "D+1")]
    D_INC,
    #[strum(to_string = "A+1")]
    A_INC,
    #[strum(to_string = "M+1")]
    M_INC,
    #[strum(to_string = "D-1")]
    D_DEC,
    #[strum(to_string = "A-1")]
    A_DEC,
    #[strum(to_string = "M-1")]
    M_DEC,
    #[strum(to_string = "D+A")]
    D_PLUS_A,
    #[strum(to_string = "D+M")]
    D_PLUS_M,
    #[strum(to_string = "D-A")]
    D_MINUS_A,
    #[strum(to_string = "D-M")]
    D_MINUS_M,
    #[strum(to_string = "A-D")]
    A_MINUS_D,
    #[strum(to_string = "M-D")]
    M_MINUS_D,
    #[strum(to_string = "D&A")]
    D_AND_A,
    #[strum(to_string = "D&M")]
    D_AND_M,
    #[strum(to_string = "D|A")]
    D_OR_A,
    #[strum(to_string = "D|M")]
    D_OR_M,
}
