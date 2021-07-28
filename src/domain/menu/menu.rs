use super::Item;

pub const CLASS: &'static str = "~rust-menu";

#[derive(Default, Clone)]
pub struct Menu {
    pub items: Vec<Item>,
    pub x: u32,
    pub y: u32,
    pub class: String
}

impl Menu {
    pub fn new() -> Self {
        Menu::default()
    }

    pub fn add(&mut self, i: Item) -> &mut Self {
        self.items.push(i);

        return self;
    }

    pub fn items(&self) -> std::slice::Iter<Item> {
        self.items.iter()
    }
}
