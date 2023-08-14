use clap::Parser;
use image::DynamicImage;
use image::imageops::FilterType;
use crate::cli::Cli;
use crate::log;

pub fn cover(input_directory: String, files: Vec<String>, filter: FilterType) {
    let Cli { size, output_file, .. } = Cli::parse();

    for file in files {
        let potential_image = input_directory.to_owned() + "/" + &file;

        if let Ok(img) = image::open(&potential_image) {
            log(&("SUCCESS: Using ./".to_owned() + &file + " for " + &input_directory + "/"));

            DynamicImage::resize_to_fill(&img, size, size, filter).save(output_file).unwrap();

            return
        }
    }
}
