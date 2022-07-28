use std::collections::HashMap;
use toml::Value;

use serde::Deserialize;

use super::basic::{Events, Layout, Style, UpdateEvents};

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default)]
    module: HashMap<String, Module>,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    extend: Option<String>,
    content: Option<String>,
    style: Option<Style>,
    layout: Option<Layout>,
    on: Option<Events>,
    update: Option<UpdateEvents>,

    #[serde(flatten, default)]
    data: HashMap<String, Value>,
}
