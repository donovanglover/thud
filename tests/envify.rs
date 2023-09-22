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

#[test]
fn envify_works_for_all_environment_variables() {
    let pwd = var("PWD").unwrap();

    assert_eq!(
        envify("$PWD/Documents"),
        format!("{pwd}/Documents"),
        "envify() works for $PWD with /"
    );
}

#[test]
fn envify_works_for_environment_variable_by_itself() {
    assert_eq!(
        envify("$PWD"),
        var("PWD").unwrap(),
        "envify() works for $PWD by itself"
    );
}
