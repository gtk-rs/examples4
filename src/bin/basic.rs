//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::new_with_label("Click me!");
    button.set_margin_top(10);
    button.set_margin_bottom(10);
    button.set_margin_start(10);
    button.set_margin_end(10);

    window.add(&button);
    window.show();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.basic"),
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
