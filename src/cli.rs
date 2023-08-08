use clap::Parser;
use std::path::PathBuf;

const LONG_ABOUT: &str = "
thud is a thumbnailer that generates thumbnails for directories.
thud stands for (thu)mbnail (d)irectory. It is written in Rust
with no external library dependencies.

thud can optionally be configured with a ~/.config/thud/config.toml.
By default, thud will create directory thumbnails for all directories based
on the files inside them. You can configure how thud creates thumbnails
with strategies.
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[arg(short, long, default_value_t = 128)]
    pub size: u32,

    #[arg(short, long)]
    pub input_directory: PathBuf,

    #[arg(short, long)]
    pub output_file: PathBuf,
}
