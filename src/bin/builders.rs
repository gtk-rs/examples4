//! # Builders Sample
//!
//! This sample demonstrates how to create a widget using the builders.
//! These allow to set construct-only properties and other construct
//! properties when creating the widget.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("First GTK+ Program")
        .default_width(350)
        .default_height(70)
        .build();

    let button = gtk::LockButtonBuilder::new()
        .margin_top(10)
        .visible(true)
        .text_lock("Lock")
        .text_unlock("Unlock")
        .build();

    window.set_child(Some(&button));

    window.show();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
