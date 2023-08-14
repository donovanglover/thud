use thud::get_config;

#[test]
fn get_config_returns_none_if_file_dne() {
    assert!(
        get_config("this-file-does-not-exist.toml").is_none(),
        "get_config() returns None if the file does not exist"
    );
}

#[test]
fn get_config_returns_none_if_invalid_toml() {
    assert!(
        get_config("./examples/invalid.toml").is_none(),
        "get_config() returns None if the toml is invalid"
    );
}

#[test]
fn get_config_returns_config() {
    assert!(
        thud::get_config("./examples/config.toml").is_some(),
        "get_config() returns a config if the file exists and is valid"
    );
}
