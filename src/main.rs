use clap::Parser;
use cli::Cli;
use thud::get_home_config;
use thud::get_filter;
use thud::log;
use thud::strategy;

mod cli;

fn main() {
    let Cli { input_directory, .. } = Cli::parse();

    if let Some(input_directory_str) = input_directory.to_str() {
        if let Some(config) = get_home_config() {
            if let Some(rules) = config.rules {
                for rule in rules {
                    if input_directory.starts_with(rule.path) {
                        if let Some(files) = rule.files {
                            let filter = rule.filter.unwrap_or("lanczos3".to_string());

                            match rule.strategy.as_str() {
                                "cover" => strategy::cover(input_directory_str.to_owned(), files, get_filter(&filter)),

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

        log("info: ~/.config/thud/config.toml not found or no rules were given");

        strategy::cover(input_directory_str.to_owned(), vec!["cover.png".to_string(), "cover.jpg".to_string()], get_filter("lanczos3"));
    }
}
