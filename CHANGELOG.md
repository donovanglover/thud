# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2023-08-14

- Rust rewrite.
- Has feature parity with 0.1.0.
- Does not use libvips.
- Thumbnail images are sharper.
- Project renamed to `thud`
- Possible to specify which files to generate thumbnails from (and in what order) with `~/.config/thud/config.toml`.
- Possible to configure thud with different *strategies* depending on a given *path*. So far, there is one strategy: cover.

## [0.1.0] - 2022-05-27

- Initial release.
- Support for `cover.png` and `cover.jpg` (in this order).
- Uses libvips.
- Written in Go.
- Binary is `go-thumbnailer`.

[unreleased]: https://github.com/donovanglover/thud/compare/0.2.0...HEAD
[0.2.0]: https://github.com/donovanglover/thud/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/donovanglover/thud/releases/tag/1.0.0
