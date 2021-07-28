use std::rc::Rc;

use gtk::prelude::*;

use crate::domain::menu;
use crate::services::sway;

pub type Anchor = gtk::Label;

pub struct Window {
    window: gtk::Window,
    anchor: Rc<Anchor>,
    model: menu::Menu,
}

impl Window {
    pub fn new(x: i32, y: i32, dom_menu: menu::Menu) -> Window {
        let window = gtk::Window::builder()
            .type_(gtk::WindowType::Toplevel)
            .title(&dom_menu.class)
            .role(&dom_menu.class)
            .opacity(0.0)
            // .app_paintable(true)
            .build();

        window.resize(x, y);

        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let label = gtk::Label::builder()
            .height_request(dom_menu.y as i32)
            .build();

        b.add(&label);

        window.add(&b);

        Window {
            window: window,
            anchor: Rc::new(label),
            model: dom_menu,
        }
    }

    pub fn show_all(&self) {
        let dom_menu = self.model.clone();

        glib::MainContext::default().push_thread_default();

        match Self::replace_sway_config(&dom_menu.class) {
            Err(e) => {
                println!("{}", e);
                gtk::main_quit();
            }
            _ => (),
        };

        let dom_menu = self.model.clone();
        let label = Rc::downgrade(&self.anchor);

        self.window.connect_show(move |_w: &gtk::Window| {
            let menu = super::menu(&dom_menu);
            menu.show_all();

            menu.connect_hide(|_| {
                gtk::main_quit();
            });

            menu.popup_at_widget(
                label.upgrade().unwrap().as_ref(),
                gdk::Gravity::SouthEast,
                gdk::Gravity::NorthEast,
                None,
            );
        });

        self.window.connect_draw(|_w: &gtk::Window, c: &cairo::Context| {
            match c.paint_with_alpha(0.0) {
                Err(e) => println!("{}", e),
                Ok(_) => ()
            }

            gtk::Inhibit(false)
        });

        self.window.show_all();
    }

    fn replace_sway_config(
        class: &String
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sway = sway::Sway::new()?;

        sway.run(&format!(
            "for_window [title=\"{}*\"] floating enable",
            class
        ))?
        .run(&format!("for_window [title=\"{}*\"] border none", class))?;

        Ok(())
    }
}
