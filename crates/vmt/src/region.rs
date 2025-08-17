#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Region {
    Constant,
    Pointer,
    Temp,
    Static,
    Local,
    Argument,
    This,
    That,
}

impl Region {
    pub(crate) fn offset(&self, static_offset: u16) -> OffsetType {
        match self {
            Region::Constant => OffsetType::Constant,
            Region::Pointer => OffsetType::Fixed(3),
            Region::Temp => OffsetType::Fixed(5),
            Region::Static => OffsetType::Fixed(16 + static_offset),
            Region::Local => OffsetType::Dynamic(1),
            Region::Argument => OffsetType::Dynamic(2),
            Region::This => OffsetType::Dynamic(3),
            Region::That => OffsetType::Dynamic(4),
        }
    }
}

#[derive(Debug)]
pub(crate) enum OffsetType {
    Constant,
    Fixed(u16),
    Dynamic(u16),
}
