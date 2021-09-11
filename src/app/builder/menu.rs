use std::path::Path;

use gtk::prelude::*;

use crate::domain::menu;

pub fn menu(dom_menu: &menu::Menu) -> gtk::Menu {
    let m = gtk::Menu::new();

    for i in dom_menu.items() {
        match i {
            menu::Item::Cmd(c) => m.append(&cmd(c)),
            menu::Item::Separator => m.append(&separator())
        };
    }

    m
}

fn cmd(c: &menu::Command) -> gtk::MenuItem {
    let item_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    if !c.icon.is_empty() {
        let image_name = &c.icon;

        let image = if Path::new(image_name).is_file() {
            gtk::Image::from_file(c.icon.as_str())
        } else {
            gtk::Image::from_icon_name(Some(image_name.as_str()), gtk::IconSize::Menu)
        };

        // TODO: Add icon
        item_box.pack_start(&image, false, false, 0);
    }

    let name = if c.name.is_empty() {
        "Unknown"
    } else {
        c.name.as_str()
    };

    item_box.pack_end(&gtk::Label::new(Some(name)), false, false, 0);

    let mi = gtk::MenuItem::new();
    mi.add(&item_box);

    let cmd = c.command.clone();

    mi.connect_activate(move |_| {
        println!("{}", cmd);
    });

    mi
}

fn separator() -> gtk::SeparatorMenuItem {
    gtk::SeparatorMenuItem::new()
}
