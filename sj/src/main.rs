#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use std::fmt::Write;
use std::fs::write;

use anyhow::{anyhow, Result};
use chrono::Local;
use clap::{arg, Args, Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;

use inquire::{InquireError, Select};
use reqwest::get;
use serde_json::{from_str, to_string};
use subjective::{
    school::{
        bells::{BellData, BellTime},
        School,
    },
    subjects::Subject,
    Subjective,
};
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
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_directory = ProjectDirs::from("school", "SubjectiveLabs", "sj")
        .ok_or_else(|| anyhow!("Couldn't find configuration directory paths. Please report this issue at {REPO}, with details about your operating system."))?;
    let config_directory = config_directory.config_dir();
    let file_path = config_directory.join(".subjective");
    match cli.command {
        Commands::Now => {
            let data = Subjective::from_config(config_directory)?;
            let now = Local::now();
            let time_now = now.time().format("%-I:%M %p").to_string().dimmed();
            let date_now = now
                .date_naive()
                .format("%A, %B %-d, %Y")
                .to_string()
                .dimmed();
            let last = data.find_first_before(now.naive_local()).ok();
            let next = data.find_first_after(now.naive_local()).ok();

            let mut output = String::new();
            writeln!(output, "{} {time_now} {date_now}", "Now".green())?;
            if let Some(BellTime {
                name: bell_name,
                bell_data,
                ..
            }) = last
            {
                match bell_data {
                    Some(BellData::Class {
                        subject_id,
                        location,
                    }) => {
                        let Subject { name: subject_name, .. } = data
                            .subjects
                            .iter()
                            .find(|subject| subject.id == *subject_id)
                            .ok_or_else(|| anyhow!("No subject found matching \"{}\". This means that your Subjective data is invalid.", subject_id))?;
                        writeln!(output, "    {} in {} {}", subject_name, location, bell_name)?;
                    }
                    Some(bell_data) => {
                        writeln!(output, "    {} {}", bell_data, bell_name)?;
                    }
                    None => {
                        writeln!(output, "    {}", bell_name)?;
                    }
                }
            }

            println!("{}", output);
        }
        Commands::Data(DataArgs { command }) => match command {
            DataCommands::Pull { server } => {
                eprintln!("Fetching schools from \"{}\"...", server);
                let response = get(format!("{}/schools.json", server)).await
                    .map_err(|_| anyhow!("Couldn't get data from Openschools. Check your internet connection and server (is \"{server}\" reachable?)."))?;
                eprintln!("Extracting text...");
                let text = response
                    .text()
                    .await
                    .map_err(|_| anyhow!("Couldn't get text from response."))?;
                eprintln!("Parsing data...");
                let schools: Vec<School> =
                    from_str(&text).map_err(|_| anyhow!("Couldn't parse schools from text."))?;
                eprintln!("Prompting user for school...");
                let school = loop {
                    let school = Select::new("Choose a school", schools.clone())
                        .with_formatter(&|school| school.value.name.clone())
                        .prompt();
                    match school {
                        Ok(school) => break school,
                        Err(
                            InquireError::OperationCanceled | InquireError::OperationInterrupted,
                        ) => {
                            return Err(anyhow!(""));
                        }
                        Err(_) => continue,
                    }
                };
                eprintln!("Creating Subjective data structures...");
                let data = Subjective::from_school(school);
                eprintln!("Serialising to JSON...");
                let json =
                    to_string(&data).map_err(|_| anyhow!("Couldn't serialise data to JSON."))?;
                eprintln!("Creating configuration directory...");
                create_dir_all(config_directory).await.map_err(|_| {
                    anyhow!(
                        "Couldn't create configuration directory at \"{}\"",
                        config_directory.display()
                    )
                })?;
                eprintln!("Writing data...");
                write(file_path.clone(), json)
                    .map_err(|_| anyhow!("Couldn't write data to \"{}\".", file_path.display()))?;
                println!("Successfully saved data to \"{}\".", file_path.display());
            }
        },
    };
    Ok(())
}
