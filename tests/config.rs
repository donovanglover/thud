use thud::get_config;

#[test]
/// Ensures that get_config(file) doesn't break if the given file does not exist.
fn get_config_returns_none_if_file_dne() {
    assert!(
        get_config("this-file-does-not-exist.toml").is_none(),
        "get_config() returns None if the file does not exist"
    );
}

#[test]
/// Ensures that get_config(file) does not break if the toml from the given file is invalid.
fn get_config_returns_none_if_invalid_toml() {
    assert!(
        get_config("./examples/invalid.toml").is_none(),
        "get_config() returns None if the toml is invalid"
    );
}

#[test]
/// Ensures that get_config(file) returns a valid config if the toml matches the struct.
fn get_config_returns_config() {
    assert!(
        thud::get_config("./examples/config.toml").is_some(),
        "get_config() returns a config if the file exists and is valid"
    );
}
