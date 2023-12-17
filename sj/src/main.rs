#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use clap::{Parser, Subcommand};
use directories::ProjectDirs;

use subjective::Subjective;

#[derive(Parser, Debug)]
#[command(
    name = "sj",
    version = "0.1.0",
    about = "Subjective's CLI tool. Manage tasks and classes from the command line."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "t", about = "View time information")]
    Time,
    #[command(alias = "d", about = "Configure Subjective data")]
    Data,
}

const REPO: &str = env!("CARGO_PKG_REPOSITORY");
fn main() {
    let cli = Cli::parse();
    let Some(config_directory) = ProjectDirs::from("school", "SubjectiveLabs", "sj") else {
        eprintln!("Couldn't find configuration directory paths. Please report this issue at {REPO}, with details about your operating system.");
        return;
    };
    let config_directory = config_directory.config_dir();
    dbg!(&cli.command);
    match cli.command {
        Commands::Time => {
            let data = match Subjective::from_config(config_directory) {
                Ok(data) => data,
                Err(error) => {
                    eprintln!("{error}");
                    return;
                }
            };
            dbg!(data);
        }
        Commands::Data => {
            
        }
    }
}
