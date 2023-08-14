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

pub fn get_config(file: String) -> Option<Config> {
    let file = Path::new(&file);

    if let Ok(config) = fs::read_to_string(file) {
        return toml::from_str(config.as_str()).unwrap();
    }

    None
}