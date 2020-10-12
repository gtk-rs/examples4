#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use glib::signal::Inhibit;
use gtk::prelude::*;
use gtk::{
    AboutDialog, AppChooserDialog, ApplicationWindow, Builder, Button, Dialog, Entry,
    FileChooserAction, FileChooserDialog, FontChooserDialog, ResponseType, SpinButton, Spinner,
    Switch, Window,
};

use std::env::args;

fn about_clicked(button: &Button, dialog: &AboutDialog) {
    if let Some(window) = button.get_root().and_then(|w| w.downcast::<Window>().ok()) {
        dialog.set_transient_for(Some(&window));
    }

    println!("Authors: {:?}", dialog.get_authors());
    println!("Artists: {:?}", dialog.get_artists());
    println!("Documenters: {:?}", dialog.get_documenters());

    // Since we only have once instance of this object with Glade, we only show/hide it.
    dialog.show();
}

fn build_ui(application: &gtk::Application) {
    println!(
        "Major: {}, Minor: {}",
        gtk::get_major_version(),
        gtk::get_minor_version()
    );
    let glade_src = include_str!("gtktest.glade");
    let builder = Builder::from_string(glade_src);

    let spinner: Spinner = builder.get_object("spinner").expect("Couldn't get spinner");
    spinner.start();

    let spin_button: SpinButton = builder
        .get_object("spin_button")
        .expect("Couldn't get spin_button");
    spin_button.connect_input(|spin_button| {
        let text = spin_button
            .get_text()
            .expect("Couldn't get text from spin_button");
        println!("spin_button_input: \"{}\"", text);
        match text.parse::<f64>() {
            Ok(value) if value >= 90. => {
                println!("circular right");
                Some(Ok(10.))
            }
            Ok(value) if value <= 10. => {
                println!("circular left");
                Some(Ok(90.))
            }
            Ok(value) => Some(Ok(value)),
            Err(_) => Some(Err(())),
        }
    });

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let button: Button = builder.get_object("button").expect("Couldn't get button");
    let entry: Entry = builder.get_object("entry").expect("Couldn't get entry");

    button.connect_clicked(clone!(@weak window, @weak entry => move |_| {
        let dialog = Dialog::new_with_buttons(Some("Hello!"),
                                              Some(&window),
                                              gtk::DialogFlags::MODAL,
                                              &[("No", ResponseType::No),
                                                ("Yes", ResponseType::Yes),
                                                ("Custom", ResponseType::Other(0))]);
        dialog.connect_response(|dialog, response| {
            // entry.set_text(&format!("Clicked {}", ret));

            dialog.close();
        });
        dialog.show();
    }));

    let button_font: Button = builder
        .get_object("button_font")
        .expect("Couldn't get button_font");
    button_font.connect_clicked(clone!(@weak window => move |_| {

        let dialog = FontChooserDialog::new(Some("Font chooser test"), Some(&window));

        dialog.show();
    }));

    let file_button: Button = builder
        .get_object("file_button")
        .expect("Couldn't get file_button");
    file_button.connect_clicked(clone!(@weak window => move |_| {

        //entry.set_text("Clicked!");
        let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                            FileChooserAction::Open, &[]);
        dialog.add_buttons(&[
            ("Open", ResponseType::Ok),
            ("Cancel", ResponseType::Cancel)
        ]);

        dialog.set_select_multiple(true);
        dialog.connect_response(|d, response| {
            if response == gtk::ResponseType::Ok {
                let files = d.get_files();
                println!("Files: {:?}", files);
            }
        });

        dialog.show();
    }));

    let app_button: Button = builder
        .get_object("app_button")
        .expect("Couldn't get app_button");
    app_button.connect_clicked(clone!(@weak window => move |_| {

        //entry.set_text("Clicked!");
        let dialog = AppChooserDialog::new_for_content_type(Some(&window),
                                                            gtk::DialogFlags::MODAL,
                                                            "sh");

        dialog.show();
    }));

    let switch: Switch = builder.get_object("switch").expect("Couldn't get switch");
    switch.connect_property_active_notify(clone!(@weak entry => move |switch| {
        if switch.get_active() {
            entry.set_text("Switch On");
        } else {
            entry.set_text("Switch Off");
        }
    }));

    let button_about: Button = builder
        .get_object("button_about")
        .expect("Couldn't get button_about");
    let dialog: AboutDialog = builder.get_object("dialog").expect("Couldn't get dialog");
    button_about.connect_clicked(move |x| about_clicked(x, &dialog));

    let event = gtk::EventControllerKey::new();
    window.add_controller(&event);
    event.set_propagation_phase(gtk::PropagationPhase::Capture);
    event.connect_key_pressed(
        clone!(@weak entry => @default-return Inhibit(false), move |_, keyval, _, keystate| {

            println!("key pressed: {} / {:?}", keyval, keystate);
            println!("text: {}", entry.get_text().expect("Couldn't get text from entry"));

            if keystate.intersects(gdk::ModifierType::CONTROL_MASK) {
                println!("You pressed Ctrl!");
            }

            Inhibit(false)
        }),
    );

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.gtktest"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
