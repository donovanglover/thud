use std::env::var;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    rule: Option<Vec<RuleConfig>>,
}

#[derive(Debug, Deserialize)]
struct RuleConfig {
    path: String,
    strategy: String,
    files: Vec<String>,
}

pub fn get_config() {
    let file = var("HOME").unwrap() + "/.config/thud/config.toml";
    let config = fs::read_to_string(file).unwrap();
    let decoded: Config = toml::from_str(config.as_str()).unwrap();
    println!("{:#?}", decoded.rule);
}
