//! # Transparent main window example
//!
//! This example demonstrates how to create a main window with a transparent background.

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Fixed};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Alpha Demo");
    window.set_default_size(500, 500);
    window.set_opacity(0.4);

    let overlay = gtk::Overlay::new();
    window.set_child(Some(&overlay));

    let fixed = Fixed::new();
    overlay.set_child(Some(&fixed));
    let button = Button::with_label("Dummy");
    button.set_size_request(100, 30);
    fixed.put(&button, 0_f64, 0_f64);

    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_draw_func(Some(Box::new(draw)));
    overlay.set_child(Some(&drawing_area));

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.transparent_main_window"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn draw(_drawing_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
    ctx.set_source_rgba(1.0, 0.0, 0.0, 1.0);
    ctx.paint();
}
