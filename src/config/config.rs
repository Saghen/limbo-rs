use serde::Deserialize;
use std::{collections::HashMap, fs};

use super::{Bar, Module};

#[derive(Deserialize, Debug)]
pub struct Config {
    include: Option<Vec<String>>,
    bar: Bar,
    #[serde(default)]
    modules: HashMap<String, Module>,
}

pub fn load_from_file(file: &str) -> Result<Config, toml::de::Error> {
    let config_raw = fs::read(file).expect("Unable to find config file");
    let config_str =
        std::str::from_utf8(&config_raw).expect("Config must be only utf-8 characters");
    let config: Config = toml::from_str(config_str)?;

    Ok(config)
}
