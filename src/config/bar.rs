use serde::Deserialize;

use super::basic::{Style, Layout, Events, UpdateEvents};

#[derive(Deserialize, Debug)]
pub struct Bar {
    monitor: Option<String>,
    pseudo_transparency: Option<bool>,
    wm_restack: Option<String>,
    side: Option<String>,

    modules: BarModules,

    style: Option<Style>,
    layout: Option<Layout>,
    on: Option<Events>,
    update: Option<UpdateEvents>,
}

#[derive(Deserialize, Debug)]
pub struct BarModules {
    start: Vec<String>,
    middle: Vec<String>,
    end: Vec<String>,
    style: Option<Style>
}
