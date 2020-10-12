//! # Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gio;
extern crate glib;
extern crate gtk;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::Builder;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let open_button: gtk::Button = builder
        .get_object("open_button")
        .expect("Couldn't get builder");
    let text_view: gtk::TextView = builder
        .get_object("text_view")
        .expect("Couldn't get text_view");

    open_button.connect_clicked(clone!(@weak window, @weak text_view => move |_| {

        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
            &[],
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);

        file_chooser.connect_response(clone!(@weak text_view => move |dialog, respnonse| {
            if respnonse == gtk::ResponseType::Ok {
                let filename = dialog
                    .get_file()
                    .expect("Couldn't get file")
                    .get_path()
                    .unwrap();
                let file = File::open(&filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view
                    .get_buffer()
                    .expect("Couldn't get window")
                    .set_text(&contents);
            }

            dialog.close();
        }));
        file_chooser.show();
    }));

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.text_viewer"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
