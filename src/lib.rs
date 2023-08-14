use std::env::var;
use std::fs;
use std::path::Path;
use serde::Deserialize;
use image::imageops::FilterType;

/// Struct for config.toml
#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

/// A Rule requires a path and a strategy
///
/// A Rule optionally includes files to target or a filter to use
#[derive(Debug, Deserialize)]
pub struct Rule {
    pub path: String,
    pub strategy: String,
    pub files: Option<Vec<String>>,
    pub filter: Option<String>,
}

/// Gets the Config from a given file location
pub fn get_config(file: &str) -> Option<Config> {
    let file = Path::new(&file);

    if let Ok(config) = fs::read_to_string(file) {
        if let Ok(toml) = toml::from_str(config.as_str()) {
            return toml
        }
    }

    None
}

/// Returns the Config located in ~/.config/thud/config.toml
pub fn get_home_config() -> Option<Config> {
    if let Ok(home) = var("HOME") {
        let config = home + "/.config/thud/config.toml";

        return get_config(config.as_str());
    }

    None
}

/// Gets the filter from a given string
///
/// Falls back to Lanczos3 if filter can't be determined from string
pub fn get_filter(filter: &str) -> FilterType {
    match filter.to_lowercase().as_str() {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        &_ => FilterType::Lanczos3,
    }
}
