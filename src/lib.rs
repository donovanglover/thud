use std::env::var;
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub path: String,
    pub strategy: String,
    pub files: Option<Vec<String>>,
    pub filter: Option<String>,
}

pub fn get_config(file: &str) -> Option<Config> {
    let file = Path::new(&file);

    if let Ok(config) = fs::read_to_string(file) {
        return toml::from_str(config.as_str()).unwrap();
    }

    None
}

pub fn get_home_config() -> Option<Config> {
    let config = var("HOME").unwrap() + "/.config/thud/config.toml";

    get_config(config.as_str())
}
