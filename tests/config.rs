use thud;

// use std::env::var;
use std::path::Path;

#[test]
fn get_config() {
    // var("HOME").unwrap() + "/.config/thud/config.toml"
    let file = Path::new("./examples/config.toml");
    dbg!(thud::get_config(file));
    assert_eq!(false, true, "get_config() returns a default config if none is present");
}
