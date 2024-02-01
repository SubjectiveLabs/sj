#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use humantime::format_duration;
use indoc::formatdoc;
use log::info;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::{fmt::Write, path::Path};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use clap::{arg, Args, Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;

use env_logger::init;
use inquire::{InquireError, Select};
use reqwest::get;
use serde_json::{from_str, to_string};
use subjective::{
    school::{bells::BellData, School},
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
    command: Option<Commands>,
    #[arg(
        short,
        long,
        help = "Use a custom time instead of the current time, which must be able to be `std::str::FromStr`'d into a `chrono::datetime::DateTime<chrono::offset::Local>`.",
        global = true,
    )]
    time: Option<DateTime<Local>>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_alias = "n", about = "View time information")]
    Now,
    #[command(visible_alias = "d", about = "Configure Subjective data")]
    Data(DataArgs),
    #[command(visible_alias = "t", about = "View timetable information")]
    Timetable(TimetableArgs),
}

#[derive(Args, Debug)]
struct DataArgs {
    #[command(subcommand)]
    command: DataCommands,
}

#[derive(Subcommand, Debug)]
enum DataCommands {
    #[command(visible_alias = "p", about = "Pull school from SubjectiveKit")]
    Pull {
        #[arg(
            short,
            long,
            help = format!("Server to pull from, defaults to \"{SUBJECTIVEKIT_URL}\""),
            default_value = SUBJECTIVEKIT_URL
        )]
        server: String,
    },

    #[command(
        visible_alias = "l",
        about = "Load school and subjects from local file"
    )]
    Load { file: PathBuf },
}

#[derive(Args, Debug)]
struct TimetableArgs {
    #[command(subcommand)]
    command: TimetableCommands,
}

#[derive(Subcommand, Debug)]
enum TimetableCommands {
    #[command(visible_alias = "s", about = "Show timetable")]
    Show,
}

const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const SUBJECTIVEKIT_URL: &str = "https://cdn.subjective.school/";

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let cli = Cli::parse();
    let config_directory =
        ProjectDirs::from("school", "SubjectiveLabs", "sj").ok_or_else(|| {
            anyhow!(formatdoc!(
                "Couldn't find configuration directory paths.
                    Please report this issue at {REPO}, with details about your operating system."
            ))
        })?;
    let config_directory = config_directory.config_dir();
    let data_file_path = config_directory.join(".subjective");
    let time = cli.time.unwrap_or_else(Local::now);
    match cli.command.unwrap_or(Commands::Now) {
        Commands::Now => {
            now(config_directory, time)?;
        }
        Commands::Data(DataArgs { command }) => match command {
            DataCommands::Pull { server } => {
                pull(&server, config_directory, &data_file_path).await?;
            }
            DataCommands::Load { file } => {
                load(&file, config_directory, &data_file_path).await?;
            }
        },
        Commands::Timetable(TimetableArgs { command }) => match command {
            TimetableCommands::Show => {
                todo!()
            }
        },
    };
    Ok(())
}

async fn pull(server: &String, config_directory: &Path, file_path: &Path) -> Result<()> {
    info!("Fetching schools from \"{}\"...", server);
    let response = get(format!("{}/schools.json", server)).await.map_err(|_| {
        anyhow!(formatdoc!(
            "Couldn't get data from Openschools.
                Check your internet connection and server (is \"{server}\" reachable?)."
        ))
    })?;
    info!("Extracting text...");
    let text = response
        .text()
        .await
        .map_err(|_| anyhow!("Couldn't get text from response."))?;
    info!("Parsing data...");
    let schools: Vec<School> =
        from_str(&text).map_err(|_| anyhow!("Couldn't parse schools from text."))?;
    info!("Prompting user for school...");
    let school = loop {
        let school = Select::new("Choose a school", schools.clone())
            .with_formatter(&|school| school.value.name.clone())
            .prompt();
        match school {
            Ok(school) => break school,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                return Err(anyhow!(""));
            }
            Err(_) => continue,
        }
    };
    save(Subjective::from_school(school), config_directory, file_path).await
}

async fn save(
    data: Subjective,
    config_directory: &Path,
    file_path: &Path,
) -> std::prelude::v1::Result<(), anyhow::Error> {
    info!("Serialising to JSON...");
    let json = to_string(&data).map_err(|_| anyhow!("Couldn't serialise data to JSON."))?;
    info!("Creating configuration directory...");
    create_dir_all(config_directory).await.map_err(|_| {
        anyhow!(
            "Couldn't create configuration directory at \"{}\"",
            config_directory.display()
        )
    })?;
    info!("Writing data...");
    write(file_path, json)
        .map_err(|_| anyhow!("Couldn't write data to \"{}\".", file_path.display()))?;
    println!("Successfully saved data to \"{}\".", file_path.display());
    Ok(())
}

async fn load(file: &PathBuf, config_directory: &Path, file_path: &Path) -> Result<()> {
    info!("Reading data from \"{}\"...", file.display());
    let json = read_to_string(file)
        .map_err(|_| anyhow!("Couldn't read data from \"{}\".", file.display()))?;
    info!("Parsing data...");
    let data: Subjective =
        from_str(&json).map_err(|_| anyhow!("Couldn't parse data from \"{}\".", file.display()))?;
    save(data, config_directory, file_path).await
}

fn now(config_directory: &Path, now: DateTime<Local>) -> Result<()> {
    let data = Subjective::from_config(config_directory)?;
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
    if let Some(bell_time) = last {
        let data = &data;
        let output = &mut output;
        let time = bell_time.time.format("%-I:%M %p").to_string().dimmed();
        let bell_name = &bell_time.name.dimmed();
        match &bell_time.bell_data {
            Some(BellData::Class {
                subject_id,
                location,
            }) => {
                let Subject {
                    name: subject_name, ..
                } = data.get_subject(*subject_id).ok_or_else(|| {
                    anyhow!(formatdoc!(
                        "No subject found matching \"{}\".
                            This means that your Subjective data is invalid.",
                        subject_id
                    ))
                })?;
                writeln!(
                    output,
                    "    {subject_name} in {location} {bell_name} {time}"
                )?;
            }
            Some(bell_data) => {
                writeln!(
                    output,
                    "    {} {bell_name} {time}",
                    format!("{bell_data}").dimmed()
                )?;
            }
            None => {
                writeln!(output, "    {bell_name} {time}")?;
            }
        }
    }
    if let Some(bell_time) = next {
        writeln!(
            output,
            "{} {} {}",
            "Upcoming".green(),
            bell_time.time.format("%-I:%M %p").to_string().dimmed(),
            format_duration(
                (now.time() - bell_time.time)
                    .abs()
                    .to_std()
                    .map_err(|_| anyhow!(
                        "Couldn't convert time to standard library `std::time::Duration`."
                    ))?
            )
            .to_string()
            .yellow()
        )?;
        let data = &data;
        let output = &mut output;
        let bell_name = &bell_time.name;
        match &bell_time.bell_data {
            Some(BellData::Class {
                subject_id,
                location,
            }) => {
                let Subject {
                    name: subject_name, ..
                } = data.get_subject(*subject_id).ok_or_else(|| {
                    anyhow!(formatdoc!(
                        "No subject found matching \"{}\".
                            This means that your Subjective data is invalid.",
                        subject_id
                    ))
                })?;
                writeln!(output, "    {subject_name} in {location} {bell_name}")?;
            }
            Some(bell_data) => {
                writeln!(
                    output,
                    "    {} {bell_name}",
                    format!("{bell_data}").dimmed()
                )?;
            }
            None => {
                writeln!(output, "    {bell_name}")?;
            }
        }
    }
    print!("{output}");
    Ok(())
}
