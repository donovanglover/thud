use clap::Parser;
use cli::Cli;
use thud::get_filter;
use thud::get_home_config;
use thud::log;

mod cli;
mod strategy;

fn main() {
    #[rustfmt::skip]
    let Cli { input_directory, .. } = Cli::parse();

    let default_files = vec!["cover.png".to_string(), "cover.jpg".to_string()];

    if let Some(input_directory_str) = input_directory.to_str() {
        if let Some(config) = get_home_config() {
            log("INFO: Using ~/.config/thud/config.toml");

            if let Some(rules) = config.rules {
                for rule in rules {
                    if input_directory.starts_with(&rule.path) {
                        #[rustfmt::skip]
                        log(&("RULES: Assigned ".to_owned() + input_directory_str + "/ to " + &rule.path));

                        let filter = rule.filter.unwrap_or("lanczos3".to_string());
                        let files = rule.files.unwrap_or(default_files);

                        match rule.strategy.as_str() {
                            #[rustfmt::skip]
                            "cover" => strategy::cover(input_directory_str.to_owned(), files, get_filter(&filter)),

                            &_ => log("warning: invalid strategy, skipping"),
                        }

                        return
                    }
                }
            }

            return;
        }

        log("INFO: Using default cover.{png,jpg}");

        #[rustfmt::skip]
        strategy::cover(input_directory_str.to_owned(), default_files, get_filter("lanczos3"));
    }
}
