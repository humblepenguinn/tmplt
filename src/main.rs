mod cli;
mod command;
mod error_macro;
mod parser;
mod progress_bar;
mod utils;
mod version;

use std::path::Path;

use clap::Parser;
use colored::Colorize;
use semver::Version;

use crate::cli::Cli;
use crate::version::get_latest_version;

fn main() {
    let latest_version = get_latest_version();

    let current_version = if let Ok(val) = Version::parse(env!("BUILD_VERSION")) {
        val
    } else {
        println!("{}: Failed to parse current version", "Error".red());
        return;
    };

    if latest_version > current_version {
        println!(
            "{}: {} -> {}",
            "New version available".yellow(),
            current_version,
            latest_version
        );
    }

    if !Path::new(&utils::get_configdir()).exists() {
        println!("{}", "Creating config directory".bold());
        if let Err(e) = std::fs::create_dir(utils::get_configdir()) {
            println!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }

        if let Err(e) = std::fs::create_dir(utils::get_configdir().join("templates")) {
            println!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    }

    let args = Cli::parse();
    args.command.run();
}
