use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    path: String,
    strategy: String,
    files: Option<Vec<String>>,
    filter: Option<String>,
}

pub fn get_config(file: String) -> Option<Config> {
    let file = Path::new(&file);

    if let Ok(config) = fs::read_to_string(file) {
        return toml::from_str(config.as_str()).unwrap();
    }

    None
}
