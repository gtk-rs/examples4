//! # Builder Signal Sample
//!
//! This sample demonstrates how to handle signals in builder

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, MessageDialog};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_signal.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");
    /* TODO: fix me
        builder
            .connect_unsafe("button1_clicked", true, move |d| {
                Box::new({
                    let dialog = dialog.clone();

                    // Return the signal handler.

                    dialog.show();
                    None
                })
            })
            .unwrap();
    }*/
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_signal"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
