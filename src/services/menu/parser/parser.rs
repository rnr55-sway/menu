use std::fmt::Debug;

use crate::domain::menu;

pub trait Error: Debug {}

pub trait Parser {
    type Error: Error;
    fn parse(&self) -> Result<menu::Menu, Self::Error>;
}
