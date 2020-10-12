//! # Accessibility example
//!
//! This sample demonstrates how to make an application more accessible.

extern crate atk;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Accessibility");

    let button = gtk::Button::with_label("Click me!");
    let label = gtk::Label::new(Some("0"));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    button.set_property_accessible_role(gtk::AccessibleRole::Button);
    label.set_property_accessible_role(gtk::AccessibleRole::Label);

    vbox.append(&button);
    vbox.append(&label);

    window.set_child(Some(&vbox));

    button.connect_clicked(move |_| {
        let value = label
            .get_text()
            .and_then(|s| u32::from_str_radix(&s, 10).ok())
            .unwrap_or(0)
            + 1;
        label.set_text(&value.to_string());
    });

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.accessibility"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        // We build the application UI.
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
