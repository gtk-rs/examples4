extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("grid.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let grid: Grid = builder.get_object("grid").expect("Couldn't get grid");

    let button6: Button = builder.get_object("button6").expect("Couldn't get button6");
    button6.connect_clicked(glib::clone!(@weak grid => move |button| {
        let layout_manager = grid
            .get_layout_manager()
            .expect("Couldn't get layout manager");
        let layout_child = layout_manager
            .get_layout_child(button)
            .expect("Couldn't get layout child")
            .dynamic_cast::<gtk::GridLayoutChild>()
            .expect("Couldn't downcast to GridLayoutChild");
        let height = layout_child.get_row_span();
        let new_height = if height == 2 { 1 } else { 2 };
        layout_child.set_row_span(new_height);
    }));

    let button7: Button = builder.get_object("button7").expect("Couldn't get button7");
    button7.connect_clicked(glib::clone!(@weak grid => move |button| {
        let layout_manager = grid
            .get_layout_manager()
            .expect("Couldn't get layout manager");
        let layout_child = layout_manager
            .get_layout_child(button)
            .expect("Couldn't get layout child")
            .dynamic_cast::<gtk::GridLayoutChild>()
            .expect("Couldn't downcast to GridLayoutChild");
        let left_attach = layout_child.get_left_attach();
        let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
        layout_child.set_left_attach(new_left_attach);
    }));

    window.show();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
