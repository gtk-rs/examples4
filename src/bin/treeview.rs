//! # TreeView Sample
//!
//! This sample demonstrates how to create a `TreeView` with either a `ListStore` or `TreeStore`.

extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, ButtonsType, CellRendererPixbuf, CellRendererText, DialogFlags,
    MessageDialog, MessageType, Orientation, TreeStore, TreeView, TreeViewColumn,
};

use std::env::args;

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("TreeView Sample");

    // left pane
    let left_tree = TreeView::new();
    let left_store = TreeStore::new(&[String::static_type()]);

    left_tree.set_model(Some(&left_store));
    left_tree.set_headers_visible(false);
    append_text_column(&left_tree);

    for i in 0..10 {
        // insert_with_values takes two slices: column indices and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        let iter = left_store.insert_with_values(None, None, &[0], &[&format!("Hello {}", i)]);

        for _ in 0..i {
            left_store.insert_with_values(Some(&iter), None, &[0], &[&"I'm a child node"]);
        }
    }

    // right pane
    let right_tree = TreeView::new();
    let right_column_types = [Pixbuf::static_type(), String::static_type()];
    let right_store = TreeStore::new(&right_column_types);
    let renderer = CellRendererPixbuf::new();
    let col = TreeViewColumn::new();

    col.set_title("Picture");
    col.pack_start(&renderer, false);

    col.add_attribute(&renderer, "pixbuf", 0);

    let renderer2 = CellRendererText::new();
    col.pack_start(&renderer2, true);
    col.add_attribute(&renderer2, "text", 1);
    let image = Pixbuf::from_file("./resources/eye.png")
        .or_else(|err| {
            let mut msg = err.to_string();
            if err.kind() == Some(glib::FileError::Noent) {
                msg.push_str(
                    "\nRelaunch this example from the same level \
                     as the `resources` folder",
                );
            }

            glib::idle_add_local(
                clone!(@weak window => @default-return Continue(false) , move || {
                    let dialog = MessageDialog::new(Some(&window), DialogFlags::MODAL,
                        MessageType::Error, ButtonsType::Ok, &msg);
                    dialog.show();
                    Continue(false)
                }),
            );

            Err(())
        })
        .ok();

    right_tree.append_column(&col);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);

    for _ in 0..10 {
        right_store.insert_with_values(
            None,
            None,
            &[0, 1],
            &[&image, &"I'm a child node with an image"],
        );
    }

    // selection and path manipulation

    let left_selection = left_tree.get_selection();
    left_selection.connect_changed(clone!(@weak right_tree => move |tree_selection| {
        let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
        let mut path = left_model.get_path(&iter).expect("Couldn't get path");
        // get the top-level element path
        while path.get_depth() > 1 {
            path.up();
        }
        right_tree.get_selection().select_path(&path);
    }));

    // display the panes

    let split_pane = gtk::Box::new(Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.append(&left_tree);
    split_pane.append(&right_tree);

    window.set_child(Some(&split_pane));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.treeview"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
