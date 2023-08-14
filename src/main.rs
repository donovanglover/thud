use clap::Parser;
use cli::Cli;
use image::DynamicImage;
use image::imageops::FilterType;
use thud::get_home_config;
use thud::get_filter;
use thud::log;
use thud::strategy;

mod cli;

fn main() {
    let Cli { size, input_directory, output_file, .. } = Cli::parse();

    if let Some(input_directory_str) = input_directory.to_str() {
        if let Some(config) = get_home_config() {
            if let Some(rules) = config.rules {
                for rule in rules {
                    if input_directory.starts_with(rule.path) {
                        if let Some(files) = rule.files {
                            let filter = rule.filter.unwrap_or("lanczos3".to_string());

                            match rule.strategy.as_str() {
                                "cover" => strategy::cover(files, get_filter(&filter)),

                                &_ => log("warning: invalid strategy, skipping"),
                            }
                        }
                    }
                }

                log("info: ~/.config/thud/config.toml was found, so those rules were used");
                return
            }

            log("info: ~/.config/thud/config.toml was found, but no rules were given")
        }

        log("info: ~/.config/thud/config.toml not found or no rules were given, so using cover strategy with cover.png and cover.jpg...");

        if let Ok(img) = image::open(input_directory_str.to_owned() + "/cover.png") {
            DynamicImage::resize_to_fill(&img, size, size, FilterType::Lanczos3).save(output_file).unwrap();
            return
        }

        if let Ok(img) = image::open(input_directory_str.to_owned() + "/cover.jpg") {
            DynamicImage::resize_to_fill(&img, size, size, FilterType::Lanczos3).save(output_file).unwrap();
            return
        }

        log("info: neither cover.png nor cover.jpg were found");
    }

    log("warning: input_directory not valid, so no work was done");
}
