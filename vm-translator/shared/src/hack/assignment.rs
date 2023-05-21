use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Assignment {
    #[strum(disabled)]
    None,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}
