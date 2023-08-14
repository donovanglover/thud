use thud::get_filter;
use image::imageops::FilterType;

#[test]
fn get_filter_returns_default_filter() {
    assert_eq!(get_filter("this-filter-dns"), FilterType::Lanczos3, "get_filter() returns default filter as Lanczos3");
}

#[test]
fn get_filter_matches_random_caps() {
    assert_eq!(get_filter("NeAResT"), FilterType::Nearest, "get_filter() disregards how a filter is capitalized");
}