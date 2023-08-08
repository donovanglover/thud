use clap::Parser;
use cli::Cli;
use image::*;
use imageops::FilterType;

mod cli;

fn main() {
    let Cli { size, input_directory, output_file } = Cli::parse();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    if let Some(input_directory) = input_directory.to_str() {
        if let Ok(img) = image::open(input_directory.to_owned() + "/cover.png") {
            DynamicImage::resize_to_fill(&img, size, size, FilterType::Nearest).save(output_file).unwrap();
            return
        }

        if let Ok(img) = image::open(input_directory.to_owned() + "/cover.jpg") {
            DynamicImage::resize_to_fill(&img, size, size, FilterType::Nearest).save(output_file).unwrap();
            return
        }
    }
}
