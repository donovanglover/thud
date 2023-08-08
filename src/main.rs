use clap::Parser;
use cli::Cli;
use image::*;
use imageops::FilterType;

mod cli;

fn main() {
    let Cli { size, input_directory, output_file } = Cli::parse();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(input_directory + "/cover.png").unwrap();
    let new_img = DynamicImage::resize_to_fill(&img, size, size, FilterType::Nearest);

    // Write the contents of this image to the Writer in PNG format.
    new_img.save(output_file).unwrap();
}
