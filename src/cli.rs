use clap::Parser;

const LONG_ABOUT: &str = "
*thud* is a thumbnailer that generates thumbnails for directories.
It was originally called go-thumbnailer and written in Go, but later
renamed to thud (for (thu)mbnail (d)irectories) and rewritten in Rust.

go-thumbnailer focused on doing one thing and doing it well: cover thumbnails.
thud adds additional functionality such as being able to customize which images
get thumbnailed, as well as how many images can be in a thumbnail.

Unlike go-thumbnailer, thud requires no external libraries to run. libvips
is no longer necessary and it's possible to enjoy the benefits of thud with
a single binary.
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[arg(short, long, default_value_t = 128)]
    pub size: u32,

    #[arg(short, long)]
    pub input_directory: String,

    #[arg(short, long)]
    pub output_file: String,
}
