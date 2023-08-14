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
        if let Ok(toml) = toml::from_str(config.as_str()) {
            return toml
        }
    }

    None
}

pub fn get_home_config() -> Option<Config> {
    if let Ok(home) = var("HOME") {
        let config = home + "/.config/thud/config.toml";

        return get_config(config.as_str());
    }

    None
}
