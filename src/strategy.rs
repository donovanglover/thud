use crate::cli::Cli;
use crate::log;
use clap::Parser;
use image::imageops::FilterType;
use image::DynamicImage;

/// The cover strategy looks for the first image in a list of potential files.
/// If one is found, that image is used as the thumbnail of the directory.
pub fn cover(input_directory: String, files: Vec<String>, filter: FilterType) {
    #[rustfmt::skip]
    let Cli { size, output_file, .. } = Cli::parse();

    for file in files {
        let potential_image = input_directory.to_owned() + "/" + &file;

        if let Ok(img) = image::open(&potential_image) {
            log(&("SUCCESS: Using ./".to_owned() + &file + " for " + &input_directory + "/"));

            #[rustfmt::skip]
            DynamicImage::resize_to_fill(&img, size, size, filter).save(output_file).unwrap();

            return;
        }
    }
}
