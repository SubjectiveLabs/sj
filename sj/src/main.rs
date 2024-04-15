#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
)]
#![allow(clippy::multiple_crate_versions)]

use humantime::format_duration;
use indoc::formatdoc;
use log::info;
use serde::{Deserialize, Serialize};
use std::iter::repeat;
use std::path::PathBuf;
use std::{fmt::Write, path::Path};
use subjective::get_current_variant;
use subjective::school::bells::BellTime;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Local};
use clap::{arg, Args, Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;

use env_logger::init;
use inquire::{InquireError, Select};
use reqwest::get;
use subjective::{school::School, Subjective};
use tokio::fs::{create_dir_all, read_to_string, write, File};

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
        global = true
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
    #[command(visible_alias = "c", about = "Configure Subjective settings")]
    Config(ConfigArgs),
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

#[derive(Args, Debug)]
struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    #[command(visible_alias = "i", about = "Initialise configuration")]
    Init,
}

const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const SUBJECTIVEKIT_URL: &str = "https://cdn.subjective.candra.dev/";

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
            now(config_directory, time).await?;
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
        Commands::Config(ConfigArgs { command }) => match command {
            ConfigCommands::Init => {
                init_config(config_directory).await?;
            }
        },
    };
    Ok(())
}

async fn init_config(config_directory: &Path) -> Result<()> {
    let config_path = config_directory.join("config.toml");
    let config = Config::default();
    let config =
        toml::to_string(&config).map_err(|_| anyhow!("Couldn't serialise configuration."))?;
    File::create(&config_path).await.map_err(|_| {
        anyhow!(
            "Couldn't create configuration file at \"{}\".",
            config_path.display()
        )
    })?;
    write(&config_path, config).await.map_err(|_| {
        anyhow!(
            "Couldn't write configuration to \"{}\".",
            config_path.display()
        )
    })?;
    println!("Successfully initialised configuration at \"{}\".", config_path.display());
    Ok(())
}

async fn pull(server: &String, config_directory: &Path, file_path: &Path) -> Result<()> {
    info!("Fetching schools from \"{}\"...", server);
    let response = get(format!("{server}/schools.json")).await.map_err(|_| {
        anyhow!(formatdoc!(
            "Couldn't get data from SubjectiveKit.
                Check your internet connection and server (is \"{server}\" reachable?)."
        ))
    })?;
    info!("Extracting text...");
    let text = response
        .text()
        .await
        .map_err(|_| anyhow!("Couldn't get text from response."))?;
    info!("Parsing data...");
    let schools: Vec<School> = serde_json::from_str(&text)
        .map_err(|error| anyhow!("Couldn't parse schools from text.\n{error}"))?;
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
    let json =
        serde_json::to_string(&data).map_err(|_| anyhow!("Couldn't serialise data to JSON."))?;
    info!("Creating configuration directory...");
    create_dir_all(config_directory).await.map_err(|_| {
        anyhow!(
            "Couldn't create configuration directory at \"{}\"",
            config_directory.display()
        )
    })?;
    info!("Writing data...");
    write(file_path, json)
        .await
        .map_err(|_| anyhow!("Couldn't write data to \"{}\".", file_path.display()))?;
    println!("Successfully saved data to \"{}\".", file_path.display());
    Ok(())
}

async fn load(file: &PathBuf, config_directory: &Path, file_path: &Path) -> Result<()> {
    info!("Reading data from \"{}\"...", file.display());
    let json = read_to_string(file)
        .await
        .map_err(|_| anyhow!("Couldn't read data from \"{}\".", file.display()))?;
    info!("Parsing data...");
    let data: Subjective = serde_json::from_str(&json)
        .map_err(|error| anyhow!("Couldn't parse data from \"{}\".\n{}", file.display(), error))?;
    save(data, config_directory, file_path).await
}

#[derive(Deserialize, Serialize)]
struct Config {
    variant_offset: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self { variant_offset: 0 }
    }
}

async fn get_config(config_directory: &Path) -> Result<Config> {
    let config_path = config_directory.join("config.toml");
    let config = read_to_string(&config_path).await.map_err(|_| {
        anyhow!(
            "Couldn't read configuration file at \"{}\".",
            config_path.display()
        )
    })?;
    toml::from_str(&config).map_err(|error| {
        anyhow!(
            "Couldn't parse configuration file at \"{}\".\n{error}",
            config_path.display()
        )
    })
}

async fn now(config_directory: &Path, now: DateTime<Local>) -> Result<()> {
    fn format(
        bell_time: &BellTime,
        output: &mut String,
        time: bool,
        data: &Subjective,
    ) -> Result<()> {
        writeln!(
            output,
            "    {}",
            (if time {
                bell_time.format_with_time(data)?
            } else {
                bell_time.format(data)?
            })
        )
        .map_err(|error| anyhow!(error))
    }
    let config = get_config(config_directory).await?;
    let data = Subjective::from_config(config_directory)?;
    let time_now = now.time().format("%-I:%M %p").to_string().dimmed();
    let date_now = now
        .date_naive()
        .format("%A, %B %-d, %Y")
        .to_string()
        .dimmed();
    let last = data
        .find_first_before(now.naive_local(), config.variant_offset)
        .ok();
    let next = data
        .find_first_after(now.naive_local(), config.variant_offset)
        .ok();

    let mut output = String::new();
    writeln!(output, "{} {time_now} {date_now}", "Now".green())?;
    if let Some(bell_time) = last {
        format(bell_time, &mut output, true, &data)?;
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
        format(bell_time, &mut output, false, &data)?;
        let next = data
            .find_all_after(now.naive_local(), config.variant_offset)
            .unwrap_or_default();
        if next.len() > 1 {
            writeln!(output, "{}", "Next".green())?;
            for bell_time in next.iter().skip(1) {
                format(bell_time, &mut output, true, &data)?;
            }
        }
    } else {
        let current_variant = get_current_variant(
            now.date_naive(),
            config.variant_offset,
            data.school.bell_times.len(),
        );
        let next_day_with_bells = repeat(data.school.bell_times.iter())
            .flatten()
            .skip(current_variant)
            .flat_map(|(_, week)| {
                week.iter()
                    .zip(["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"])
            })
            .skip(now.weekday().num_days_from_sunday() as usize)
            .find(|(day, _)| !day.is_empty());
        if let Some((day, weekday)) = next_day_with_bells {
            writeln!(output, "{} {}", "Upcoming".green(), weekday.dimmed())?;
            for bell_time in day {
                format(bell_time, &mut output, true, &data)?;
            }
        }
    }
    print!("{output}");
    Ok(())
}
