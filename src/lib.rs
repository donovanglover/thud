use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    rules: Option<Vec<Rules>>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    path: String,
    strategy: String,
    files: Vec<String>,
}

pub fn get_config(file: &Path) -> Option<Vec<Rules>> {
    let config = fs::read_to_string(file).unwrap();
    let decoded: Config = toml::from_str(config.as_str()).unwrap();
    decoded.rules
}
