use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, EnumString)]
pub enum Assignment {
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}
