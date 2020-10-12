//! Simple drag and drop example
//!
//! Ported over from example code:
//! https://developer.gnome.org/gtkmm-tutorial/stable/sec-dnd-example.html.en

extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    // Configure button as drag source for text
    let button = gtk::Button::with_label("Drag here");
    let targets = gdk::ContentFormats::new(&["STRING", "text/plain"]);
    button.drag_source_set(
        gdk::ModifierType::MODIFIER_MASK,
        Some(&targets),
        gdk::DragAction::COPY,
    );
    button.connect_drag_data_get(|_, _, s| {
        let data = "I'm data!";
        s.set_text(data);
    });
    button.set_property_expand(true);

    // Configure label as drag destination to receive text
    let label = gtk::Label::new(Some("Drop here"));
    label.drag_dest_set(
        gtk::DestDefaults::ALL,
        Some(&targets),
        gdk::DragAction::COPY,
    );
    label.connect_drag_data_received(|w, _, s| {
        w.set_text(&s.get_text().expect("Couldn't get text"));
    });

    // Stack the button and label horizontally
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.append(&button);
    hbox.append(&label);

    // Finish populating the window and display everything
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Simple Drag and Drop Example");
    window.set_default_size(200, 100);
    window.set_child(Some(&hbox));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.drag_and_drop"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
