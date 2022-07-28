extern crate gio;
extern crate glib;
extern crate gtk;
extern crate pango;
extern crate toml;
extern crate serde;

mod config;

use config::config::load_from_file;
use gtk::{prelude::*, Application, ApplicationWindow, CenterBox, Label, Orientation};
use pango::{AttrInt, AttrList, Attribute, Weight};
use std::fs;

use toml::Value;

const APP_ID: &str = "org.saghen.limbo";

fn main() {
    let config = load_from_file("example-config/bar.toml").unwrap();
    println!("{:?}", config);

    // let app = Application::builder().application_id(APP_ID).build();
    // app.connect_activate(build_ui);
    // app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(2560)
        .default_height(30)
        .resizable(false)
        .name("foo")
        .title("bar")
        .build();

    let start_box = gtk::Box::new(Orientation::Horizontal, 0);
    let center_box = gtk::Box::new(Orientation::Horizontal, 0);
    let end_box = gtk::Box::new(Orientation::Horizontal, 0);

    let unified_box = CenterBox::new();
    unified_box.set_start_widget(Some(&start_box));
    unified_box.set_center_widget(Some(&center_box));
    unified_box.set_end_widget(Some(&end_box));

    let font_description = Attribute::from(AttrInt::new_weight(Weight::Bold));
    let attr_list = AttrList::new();
    attr_list.insert(font_description);

    start_box.append(
        &Label::builder()
            .attributes(&attr_list)
            .label("Start")
            .build(),
    );
    center_box.append(&Label::new(Some("Center")));
    end_box.append(&Label::new(Some("End")));

    win.set_child(Some(&unified_box));

    // Show the window
    win.present();
}
