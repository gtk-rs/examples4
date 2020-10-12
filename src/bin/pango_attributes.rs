//! # Pango text attributes
//!
//! This sample demonstrates how to use various attributes on labels text.

extern crate gio;
extern crate gtk;
extern crate pango;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Pango text attributes");
    window.set_default_size(350, 70);

    let label = gtk::Label::new(Some("Some text"));
    let attr_list = pango::AttrList::new();

    let mut attr =
        pango::Attribute::new_background(65535, 0, 0).expect("Couldn't create new background");
    attr.set_start_index(0);
    attr.set_end_index(2);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_underline(pango::Underline::Single)
        .expect("Couldn't create new underline");
    attr.set_start_index(1);
    attr.set_end_index(4);
    attr_list.insert(attr);

    let mut attr =
        pango::Attribute::new_strikethrough(true).expect("Couldn't create new strikethrough");
    attr.set_start_index(5);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_scale(1.2).expect("Couldn't create new scale");
    attr.set_start_index(6);
    attr_list.insert(attr);

    label.set_attributes(Some(&attr_list));

    window.set_child(Some(&label));

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.pango_attributes"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
