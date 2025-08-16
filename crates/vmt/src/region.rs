#[derive(Debug)]
pub(crate) enum RegionType {
    Constant,
    Fixed(u16),
    Dynamic(u16),
}

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
    pub(crate) fn offset(&self) -> RegionType {
        match self {
            Region::Constant => RegionType::Constant,
            Region::Pointer => RegionType::Fixed(3),
            Region::Temp => RegionType::Fixed(5),
            Region::Static => RegionType::Fixed(16),
            Region::Local => RegionType::Dynamic(1),
            Region::Argument => RegionType::Dynamic(2),
            Region::This => RegionType::Dynamic(3),
            Region::That => RegionType::Dynamic(4),
        }
    }
}
