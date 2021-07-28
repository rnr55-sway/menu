extern crate cairo;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate log;
extern crate serde_json;
extern crate stderrlog;

mod app;
mod domain;
mod services;

use services::menu::parser::json::Json;

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();

    let r = std::io::stdin();

    let p = Json::new(r);

    let main_app = app::App::init(p).unwrap();

    main_app.run();
}
