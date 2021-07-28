use std::convert::TryFrom;
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum Gravity {
    NorthWest,
    West,
    NorthEast,
    East,
}

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("Wrong value (`{0}`) of gravity")]
    WrongValue(&'a str),
}

impl<'a> TryFrom<&'a str> for Gravity {
    type Error = Error<'a>;
    fn try_from(s: &str) -> Result<Self, Error> {
        match s {
            "west" => Ok(Gravity::West),
            "east" => Ok(Gravity::East),
            "north-west" => Ok(Gravity::NorthWest),
            "north-east" => Ok(Gravity::NorthEast),
            "" => Ok(Gravity::default()),
            _ => Err(Error::WrongValue(s)),
        }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity::NorthEast
    }
}
