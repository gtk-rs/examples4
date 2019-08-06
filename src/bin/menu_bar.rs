//! # MenuBar Sample
//!
//! This sample demonstrates how to use Menus/MenuBars and MenuItems in Windows.
//!
//! /!\ This is different from the system menu bar (which are preferred) available in `gio::Menu`!

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, Image, Label, Menu,
    MenuBar, MenuItem, WindowPosition,
};

use std::env::args;

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

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("MenuBar example");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let file_image = Image::new_from_file("resources/file.png");
    let file_label = Label::new(Some("File"));
    let folder_item = MenuItem::new();
    let folder_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let folder_image = Image::new_from_icon_name(Some("folder-music-symbolic"));
    let folder_label = Label::new(Some("Folder"));
    let check_item = CheckMenuItem::new_with_label("Click me!");

    file_box.add(&file_image);
    file_box.add(&file_label);
    file_item.add(&file_box);
    folder_box.add(&folder_image);
    folder_box.add(&folder_label);
    folder_item.add(&folder_box);
    menu.append(&file_item);
    menu.append(&folder_item);
    menu.append(&check_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    let other_menu = Menu::new();
    let sub_other_menu = Menu::new();
    let other = MenuItem::new_with_label("Another");
    let sub_other = MenuItem::new_with_label("Sub another");
    let sub_other2 = MenuItem::new_with_label("Sub another 2");
    let sub_sub_other2 = MenuItem::new_with_label("Sub sub another 2");
    let sub_sub_other2_2 = MenuItem::new_with_label("Sub sub another2 2");

    sub_other_menu.append(&sub_sub_other2);
    sub_other_menu.append(&sub_sub_other2_2);
    sub_other2.set_submenu(Some(&sub_other_menu));
    other_menu.append(&sub_other);
    other_menu.append(&sub_other2);
    other.set_submenu(Some(&other_menu));
    menu_bar.append(&other);

    let window_weak = window.downgrade();
    quit.connect_activate(move |_| {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    // It isn't available directly through gdk::ModifierType, since it has
    // different values on different platforms.
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let label = Label::new(Some("MenuBar example"));
    label.set_property_expand(true);

    v_box.add(&menu_bar);
    v_box.add(&label);
    window.add(&v_box);
    window.show();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["gtk-rs developers"]);
        p.set_website_label("gtk-rs");
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
    check_item.connect_toggled(|w| {
        w.set_label(if w.get_active() {
            "Checked"
        } else {
            "Unchecked"
        });
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
