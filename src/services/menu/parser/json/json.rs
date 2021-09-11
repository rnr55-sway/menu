use std::cell::RefCell;

use serde_json;

use crate::domain::menu;

use super::super::*;
use super::Error;

pub struct Json<R: std::io::Read> {
    reader: RefCell<R>
}

impl<R: std::io::Read> Json<R> {
    pub fn new(reader: R) -> Json<R> {
        Json {
            reader: RefCell::new(reader)
        }
    }
}

impl From<serde_json::Value> for menu::Menu {
    fn from(v: serde_json::Value) -> menu::Menu {
        let mut m = menu::Menu::new();

        m.x = v["x"].as_u64().unwrap_or_default() as u32;
        m.y = v["y"].as_u64().unwrap_or_default() as u32;
        m.class = String::from(v["class"].as_str().unwrap_or(menu::CLASS));

        m.items = match v["items"].as_array() {
            Some(v) => v
                .into_iter()
                .map(|v| match v["name"].as_str() {
                    Some(s) => menu::Item::Cmd(menu::Command {
                        name: String::from(s),
                        icon: String::from(v["icon"].as_str().unwrap_or_default()),
                        command: String::from(v["cmd"].as_str().unwrap_or_default())
                    }),
                    None => menu::Item::Separator
                })
                .collect(),
            None => vec![]
        };

        m
    }
}

impl<R: std::io::Read> Parser for Json<R> {
    type Error = Error;
    fn parse(&self) -> Result<menu::Menu, Self::Error> {
        let mut r = self.reader.try_borrow_mut()?;

        let value: serde_json::Value = serde_json::from_reader(r.by_ref())?;
        Ok(menu::Menu::from(value))
    }
}
