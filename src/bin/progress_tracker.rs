//! Track progress with a background thread and a channel.

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use glib::signal::Inhibit;
use std::cell::{Cell, RefCell};
use std::env::args;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

pub fn main() {
    glib::set_program_name(Some("Progress Tracker"));

    let application = gtk::Application::new(
        Some("com.github.progress-tracker"),
        gio::ApplicationFlags::empty(),
    )
    .expect("initialization failed");

    application.connect_startup(|app| {
        let application = Application::new(app);

        let application_container = RefCell::new(Some(application));
        app.connect_shutdown(move |_| {
            let application = application_container
                .borrow_mut()
                .take()
                .expect("Shutdown called multiple times");
            // Here we could do whatever we need to do for shutdown now
            drop(application);
        });
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

pub struct Application {
    pub widgets: Rc<Widgets>,
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let app = Application {
            widgets: Rc::new(Widgets::new(app)),
        };

        app.connect_progress();

        app
    }

    fn connect_progress(&self) {
        let widgets = Rc::downgrade(&self.widgets);
        let active = Rc::new(Cell::new(false));
        self.widgets.main_view.button.connect_clicked(move |_| {
            let widgets = upgrade_weak!(widgets);
            if active.get() {
                return;
            }

            active.set(true);

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            thread::spawn(move || {
                for v in 1..=10 {
                    let _ = tx.send(Some(v));
                    thread::sleep(Duration::from_millis(500));
                }
                let _ = tx.send(None);
            });

            let active = active.clone();
            let widgets = widgets.clone();
            rx.attach(None, move |value| match value {
                Some(value) => {
                    widgets
                        .main_view
                        .progress
                        .set_fraction(f64::from(value) / 10.0);

                    if value == 10 {
                        widgets
                            .view_stack
                            .set_visible_child(&widgets.complete_view.container);

                        let widgets = widgets.clone();
                        glib::timeout_add_local(1500, move || {
                            widgets.main_view.progress.set_fraction(0.0);
                            widgets
                                .view_stack
                                .set_visible_child(&widgets.main_view.container);
                            glib::Continue(false)
                        });
                    }

                    glib::Continue(true)
                }
                None => {
                    active.set(false);
                    glib::Continue(false)
                }
            });
        });
    }
}

pub struct Widgets {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub view_stack: gtk::Stack,
    pub main_view: MainView,
    pub complete_view: CompleteView,
}

impl Widgets {
    pub fn new(application: &gtk::Application) -> Self {
        let complete_view = CompleteView::new();
        let main_view = MainView::new();

        let view_stack = gtk::Stack::new();
        view_stack.set_property_margin(6);
        view_stack.set_vexpand(true);
        view_stack.set_hexpand(true);
        view_stack.add(&main_view.container);
        view_stack.add(&complete_view.container);

        let header = Header::new();

        let window = gtk::ApplicationWindow::new(application);
        window.set_icon_name(Some("package-x-generic"));
        window.set_property_window_position(gtk::WindowPosition::Center);
        window.set_titlebar(Some(&header.container));
        window.add(&view_stack);
        window.show();
        window.set_default_size(500, 250);
        window.connect_close_request(move |window| {
            window.destroy();
            Inhibit(false)
        });

        Widgets {
            window,
            header,
            view_stack,
            main_view,
            complete_view,
        }
    }
}

pub struct Header {
    container: gtk::HeaderBar,
}

impl Header {
    pub fn new() -> Self {
        let container = gtk::HeaderBar::new();
        container.set_title(Some("Progress Tracker"));
        container.set_show_title_buttons(true);

        Header { container }
    }
}

pub struct CompleteView {
    pub container: gtk::Grid,
}

impl CompleteView {
    pub fn new() -> Self {
        let label = gtk::Label::new(None);
        label.set_markup("Task complete");
        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);
        label.set_vexpand(true);
        label.set_hexpand(true);

        let container = gtk::Grid::new();
        container.set_vexpand(true);
        container.set_hexpand(true);
        container.add(&label);

        CompleteView { container }
    }
}

pub struct MainView {
    pub container: gtk::Grid,
    pub progress: gtk::ProgressBar,
    pub button: gtk::Button,
}

impl MainView {
    pub fn new() -> Self {
        let progress = gtk::ProgressBar::new();
        progress.set_text(Some("Progress Bar"));
        progress.set_show_text(true);
        progress.set_hexpand(true);

        let button = gtk::Button::new();
        button.set_label("start");
        button.set_halign(gtk::Align::Center);

        let container = gtk::Grid::new();
        container.attach(&progress, 0, 0, 1, 1);
        container.attach(&button, 0, 1, 1, 1);
        container.set_row_spacing(12);
        container.set_property_margin(6);
        container.set_vexpand(true);
        container.set_hexpand(true);

        MainView {
            container,
            progress,
            button,
        }
    }
}
