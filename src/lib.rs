use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    rules: Option<Vec<Rules>>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    path: String,
    strategy: String,
    files: Option<Vec<String>>,
    filter: Option<String>,
}

pub fn get_config(file: &Path) -> Option<Config> {
    if let Ok(config) = fs::read_to_string(file) {
        toml::from_str(config.as_str()).unwrap()
    } else {
        None
    }
}
