use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub(crate) enum RegionType {
    Constant,
    Fixed(u16),
    Dynamic(u16),
}

#[derive(Debug)]
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

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Invalid region; line={0}")]
pub(crate) struct ParseRegionErr(String);

impl FromStr for Region {
    type Err = ParseRegionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "constant" => Ok(Region::Constant),
            "pointer" => Ok(Region::Pointer),
            "temp" => Ok(Region::Temp),
            "static" => Ok(Region::Temp),
            "local" => Ok(Region::Local),
            "argument" => Ok(Region::Argument),
            "this" => Ok(Region::This),
            "that" => Ok(Region::That),
            _ => Err(ParseRegionErr(s.to_owned())),
        }
    }
}
