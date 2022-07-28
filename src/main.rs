extern crate gio;
extern crate glib;
extern crate gtk;
extern crate pango;
extern crate toml;

mod config;

use gtk::{prelude::*, Application, ApplicationWindow, CenterBox, Label, Orientation};
use pango::{AttrInt, AttrList, Attribute, Weight};
use std::fs;

use toml::Value;

const APP_ID: &str = "org.saghen.limbo";

fn main() {
    let config_raw = fs::read("example-config.toml").expect("Unable to find config file");
    let config_str =
        std::str::from_utf8(&config_raw).expect("Config must be only utf-8 characters");
    let config = config_str
        .parse::<Value>()
        .expect("Failed to parse config file");

    println!("{}", config["wm"]);

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
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
