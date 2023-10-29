# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2023-10-29

### New Features

- Added colors to `--help`. Optionally disable with `NO_COLOR=1`.

## [0.3.0] - 2023-09-21

### Breaking Changes

#### `input-directory` and `output-file` are now arguments

This makes it easier to tell that these values are required and follows other command line tools that deal with input/output.

Before:

```bash
thud --size 256 --input-directory /path/to/directory --output-file thumbnail.png
```

After:

```bash
thud --size 256 /path/to/directory thumbnail.png
```

### New Features

#### Environment variables supported in config file

It is now possible to use `$HOME` and other environment variables at the start of a `path` in a rule.

```toml
[[rules]]
path = "$HOME/Videos"
strategy = "cover"
files = [
  "cover.webp"
]
```

For convenience, `~` is also supported.

```toml
[[rules]]
path = "~/Documents"
strategy = "cover"
```

## [0.2.0] - 2023-08-14

- Rust rewrite.
- Has feature parity with 0.1.0.
- Does not use libvips.
- Project renamed to `thud`
- Ability to customize the image filter used by the thumbnail. The default `Lanczos3` is high quality.
- Possible to specify which files to generate thumbnails from (and in what order) with `~/.config/thud/config.toml`.
- Possible to configure thud with different *strategies* depending on a given *path*. So far, there is one strategy: cover.

## [0.1.0] - 2022-05-27

- Initial release.
- Support for `cover.png` and `cover.jpg` (in this order).
- Uses libvips.
- Written in Go.
- Binary is `go-thumbnailer`.

[unreleased]: https://github.com/donovanglover/thud/compare/0.3.1...HEAD
[0.3.1]: https://github.com/donovanglover/thud/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/donovanglover/thud/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/donovanglover/thud/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/donovanglover/thud/releases/tag/1.0.0
