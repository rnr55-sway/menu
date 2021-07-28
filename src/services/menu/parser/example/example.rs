
use crate::domain::menu;

use super::super::*;
use super::Error;

pub struct Example();

impl Example {
    #[allow(dead_code)]
    pub fn new() -> Example {
        Example()
    }
}

impl Parser for Example {
    type Error = Error;

    fn parse(&self) -> Result<menu::Menu, Self::Error> {
        let mut m = menu::Menu::new();

        m.x = 1280;

        m.add(menu::Item::Cmd(menu::Command {
            icon: String::new(),
            name: String::from("Save to log"),
            command: String::from("test"),
        }))
        .add(menu::Item::Separator)
        .add(menu::Item::Cmd(menu::Command {
            icon: String::new(),
            name: String::from("Save to log 2"),
            command: String::from("test2"),
        }));

        Ok(m)
    }
}
