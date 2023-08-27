use image::imageops::FilterType;
use thud::get_filter;

#[test]
/// Ensures that a decent default filter is used if the one provided is invalid.
fn get_filter_returns_default_filter() {
    assert_eq!(
        get_filter("this-filter-dne"),
        FilterType::Lanczos3,
        "get_filter() returns default filter as Lanczos3"
    );
}

#[test]
/// Ensures that the correct filter is used despite how a user capitalizes it.
///
/// Useful for user experience since users may not remember if a filter was "Nearest" or "nearest".
fn get_filter_matches_random_caps() {
    assert_eq!(
        get_filter("NeAResT"),
        FilterType::Nearest,
        "get_filter() disregards how a filter is capitalized"
    );
}
