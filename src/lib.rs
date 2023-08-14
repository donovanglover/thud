use std::fs;
use std::path::Path;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = r#"
[[rules]]
path = "/"
strategy = "cover"
files = [
  "cover.png",
  "cover.jpg"
]
"#;

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

pub fn get_config(file: &Path) -> Config {
    if let Ok(config) = fs::read_to_string(file) {
        toml::from_str(config.as_str()).unwrap()
    } else {
        toml::from_str(DEFAULT_CONFIG).unwrap()
    }
}
