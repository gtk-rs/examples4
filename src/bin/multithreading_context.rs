extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::thread;
use std::time::Duration;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Multithreading GTK+ Program");
    window.set_default_size(600, 400);

    let text_view = gtk::TextView::new();
    let scroll = gtk::ScrolledWindow::new();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_child(Some(&text_view));

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        for i in 1..100 {
            // do long work
            thread::sleep(Duration::from_millis(50));
            // send result to channel
            tx.send(format!("#{} Text from another thread.", i))
                .expect("Couldn't send data to channel");
            // receiver will be run on the main thread
        }
    });

    // Attach receiver to the main context and set the text buffer text from here
    let text_buffer = text_view
        .get_buffer()
        .expect("Couldn't get buffer from text_view");
    rx.attach(None, move |text| {
        text_buffer.set_text(&text);

        glib::Continue(true)
    });

    window.set_child(Some(&scroll));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.multithreading_context"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
