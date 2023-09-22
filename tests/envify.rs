use std::env::var;
use thud::envify;

#[test]
fn envify_changes_home_to_environment_variable() {
    let home = var("HOME").unwrap();

    assert_eq!(
        envify("$HOME/Documents"),
        format!("{home}/Documents"),
        "envify() works for $HOME"
    );

    assert_eq!(
        envify("~/Pictures"),
        format!("{home}/Pictures"),
        "envify() works for ~"
    );
}
