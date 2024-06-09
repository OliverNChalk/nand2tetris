use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, EnumString)]
pub enum Branch {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}
