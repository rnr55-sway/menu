use crate::domain::menu::gravity::Gravity;

use crate::services::menu::parser;

use super::builder;
use super::Error;

pub struct App {
    root: builder::Window
}

impl Into<gdk::Gravity> for Gravity {
    fn into(self) -> gdk::Gravity {
        use gdk::Gravity::*;

        match self {
            Gravity::West => West,
            Gravity::East => East,
            Gravity::NorthWest => NorthWest,
            Gravity::NorthEast => NorthEast
        }
    }
}

impl App {
    pub fn init<P: parser::Parser>(parser: P) -> Result<App, Error<P::Error>> {
        let dom_menu = parser.parse()?;

        gtk::init()?;

        let screen = gdk::Screen::default().ok_or(Error::DefaultScreen)?;
        let provider = gtk::CssProvider::new();

        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        let display = gdk::Display::default().ok_or(Error::DefaultDisplay)?;

        let geometry = display
            .monitor_at_point(dom_menu.x as i32, dom_menu.y as i32)
            .ok_or(Error::MonitorNotFound)?
            .geometry();

        let mut a = App {
            root: builder::Window::new(geometry.width, geometry.height, dom_menu)
        };

        a.connect();

        Ok(a)
    }

    pub fn run(self) {
        self.root.show_all();

        gtk::main();
    }

    fn connect(&mut self) {}
}
