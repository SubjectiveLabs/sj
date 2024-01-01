#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use std::fs::write;

use anyhow::Result;
use chrono::Local;
use clap::{arg, Args, Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;

use indoc::printdoc;
use inquire::{InquireError, Select};
use reqwest::get;
use serde_json::{from_str, to_string};
use subjective::{school::School, Subjective};
use tokio::fs::create_dir_all;

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
    #[command(visible_alias = "n", about = "View time information")]
    Now,
    #[command(visible_alias = "d", about = "Configure Subjective data")]
    Data(DataArgs),
}

#[derive(Args, Debug)]
struct DataArgs {
    #[command(subcommand)]
    command: DataCommands,
}

#[derive(Subcommand, Debug)]
enum DataCommands {
    #[command(visible_alias = "p", about = "Pull from Subjective Openschools")]
    Pull {
        #[arg(
            short,
            long,
            help = format!("Server to pull from, defaults to \"{OPENSCHOOLS_URL}\""),
            default_value = OPENSCHOOLS_URL
        )]
        server: String,
    },
}

const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const OPENSCHOOLS_URL: &str = "https://cdn.subjective.school/";

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let Some(config_directory) = ProjectDirs::from("school", "SubjectiveLabs", "sj") else {
        eprintln!("Couldn't find configuration directory paths. Please report this issue at {REPO}, with details about your operating system.");
        return;
    };
    let config_directory = config_directory.config_dir();
    let file_path = config_directory.join(".subjective");
    match cli.command {
        Commands::Now => {
            let data = match Subjective::from_config(config_directory) {
                Ok(data) => data,
                Err(error) => {
                    eprintln!("{error}");
                    return;
                }
            };
            let now = Local::now();
            let time_now = now.time().format("%-I:%M %p").to_string().dimmed();
            let date_now = now.date_naive().format("%A, %B %-d, %Y").to_string().dimmed();
            printdoc! {"
                {} {time_now} {date_now}
                    
            ", "Now".green()}
        }
        Commands::Data(DataArgs { command }) => match command {
            DataCommands::Pull { server } => {
                eprintln!("Fetching schools from \"{}\"...", server);
                let Ok(response) = get(format!("{}/schools.json", server)).await else {
                    eprintln!("Couldn't get data from Openschools. Check your internet connection and server (is \"{server}\" reachable?).");
                    return;
                };
                eprintln!("Extracting text...");
                let Ok(text) = response.text().await else {
                    eprintln!("Couldn't get text from response.");
                    return;
                };
                eprintln!("Parsing data...");
                let Ok(schools): Result<Vec<School>, _> = from_str(&text) else {
                    eprintln!("Couldn't parse schools from text.");
                    return;
                };
                eprintln!("Prompting user for school...");
                let school = loop {
                    let school = Select::new("Choose a school", schools.clone())
                        .with_formatter(&|school| school.value.name.clone())
                        .prompt();
                    match school {
                        Ok(school) => break school,
                        Err(
                            InquireError::OperationCanceled | InquireError::OperationInterrupted,
                        ) => return,
                        Err(_) => continue,
                    }
                };
                eprintln!("Creating Subjective data structures...");
                let data = Subjective::from_school(school);
                eprintln!("Serialising to JSON...");
                let Ok(json) = to_string(&data) else {
                    eprintln!("Couldn't serialise data to JSON.");
                    return;
                };
                eprintln!("Creating configuration directory...");
                let Ok(()) = create_dir_all(config_directory).await else {
                    eprintln!(
                        "Couldn't create configuration directory at \"{}\"",
                        config_directory.display()
                    );
                    return;
                };
                eprintln!("Writing data...");
                let Ok(()) = write(file_path.clone(), json) else {
                    eprintln!("Couldn't write data to \"{}\".", file_path.display());
                    return;
                };
                println!("Successfully saved data to \"{}\".", file_path.display());
            }
        },
    };
}
