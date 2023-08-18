use clap::Parser;
use std::path::PathBuf;

const LONG_ABOUT: &str = "
thud is a thumbnailer that generates thumbnails for directories.
thud stands for (thu)mbnail (d)irectory. It is written in Rust
with no external library dependencies.

thud can optionally be configured with a ~/.config/thud/config.toml.
By default, thud will create directory thumbnails for all directories
that have a cover.{png,jpg}. You can configure how thud creates
thumbnails with strategies.

For an example config file, see:
https://github.com/donovanglover/thud/blob/master/examples/config.toml

For a list of available strategies, see:
https://github.com/donovanglover/thud/blob/master/src/strategy.rs
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    /// Thumbnail size
    #[arg(short, long, value_name = "INTEGER", default_value_t = 128)]
    pub size: u16,

    /// Directory to thumbnail
    pub input_directory: PathBuf,

    /// Where to save the output image
    pub output_file: PathBuf,

    /// Print what thud is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
