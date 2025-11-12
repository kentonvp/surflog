use anyhow::Result;
use chrono::NaiveDateTime;
use clap::{Args, Parser, Subcommand};
use log::{debug, error};

use crate::database::{Database, database_path, models::session};
use crate::input;

const GREEN: &'static str = "\x1b[32m";
const YELLOW: &'static str = "\x1b[33m";
const RESET: &'static str = "\x1b[0m";

/// A simple command-line tool to track surf sessions.
#[derive(Parser)]
#[command(
    author="Kenton Van Peursem",
    version="0.0.1",
    about="CLI tool to track surf sessions",
    long_about = None
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Add a new surf session
    Add(Add),

    /// Delete a surf session
    Delete(Delete),

    /// List surf sessions
    List(List),

    /// Display configuration elements
    Config,
}

#[derive(Args, Debug)]
pub(crate) struct Delete {
    /// `id` of the surf session to delete
    pub(crate) id: u64,
}

#[derive(Args, Debug)]
pub(crate) struct List {
    /// Optional location of surf sessions to list
    pub(crate) location: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct Add;

pub(crate) fn get_args() -> Cli {
    Cli::parse()
}

pub(crate) fn run() -> Result<()> {
    let args = get_args();

    let db = Database::new(&database_path()?);

    match args.cmd {
        Command::Add(_opts) => {
            debug!("Building a new session...");
            let mut builder = session::SessionBuilder::builder();

            // Location -------------------------------------------------------
            let previous = session::get_last_location(&db);
            loop {
                let mut prompt = format!("{GREEN}Where");
                if let Some(loc) = &previous {
                    prompt.push_str(&format!("{YELLOW} (previous: {loc})"));
                }
                prompt.push_str(&format!("{GREEN}: {RESET}"));

                let location = input::get_input(&prompt);
                if location.trim().is_empty()
                    && let Some(default) = previous
                {
                    builder.location(default);
                    break;
                } else if !location.trim().is_empty() {
                    builder.location(location);
                    break;
                } else {
                    println!("Location cannot be empty.");
                }
            }

            // Datetime -------------------------------------------------------
            loop {
                let date = input::get_input(&format!(
                    "{GREEN}When{YELLOW} (YYYY-MM-DD HH:MM){GREEN}:{RESET} "
                ));
                if let Ok(date) =
                    NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M")
                {
                    builder.date(date);
                    break;
                } else {
                    println!(
                        "Invalid date format. Please use 'YYYY-MM-DD HH:MM'."
                    );
                }
            }

            // Duration -------------------------------------------------------
            loop {
                let duration = input::get_input(&format!(
                    "{GREEN}Duration {YELLOW}(minutes){GREEN}: {RESET}"
                ));
                if let Ok(duration) = duration.trim().parse::<u16>() {
                    builder.duration(duration);
                    break;
                } else {
                    println!("Please enter a valid number for duration.");
                }
            }

            // Rating ---------------------------------------------------------
            loop {
                let rating = input::get_input(&format!(
                    "{GREEN}Rating {YELLOW}(1-10){GREEN}: {RESET}"
                ));
                if let Ok(rating) = rating.trim().parse::<u8>() {
                    if (1..=10).contains(&rating) {
                        builder.rating(rating);
                        break;
                    } else {
                        println!("Rating must be between 1 and 10.");
                    }
                } else {
                    println!("Please enter a valid number for rating.");
                }
            }

            // Wave height ----------------------------------------------------
            loop {
                let wave_height = input::get_input(&format!(
                    "{GREEN}Approx wave height {YELLOW}(feet){GREEN}: {RESET}"
                ));
                if let Ok(wave_height) = wave_height.trim().parse::<f32>() {
                    builder.wave_height(wave_height);
                    break;
                } else {
                    println!("Please enter a valid number for wave height.");
                }
            }

            // Build and write session to database
            let session = builder.build().expect("Failed to build session");
            debug!("New session: {:?}", session);

            // Insert session and log if error
            if let Err(e) = session::insert(&db, &session) {
                error!("Failed to insert session: {}", e);
            }
        }
        Command::Delete(del_opts) => {
            debug!("Delete a session...");
            session::delete(&db, del_opts.id);
        }
        Command::List(list_opts) => {
            debug!("Listing all sessions...");
            // Display all sessions
            let sessions = match list_opts.location {
                Some(loc) => session::get_by_location(&db, &loc),
                None => session::get_all(&db),
            };

            if let Ok(sessions) = sessions {
                println!(
                    "|{:^4}|{:^20}|{:^18}|{:^10}|{:^8}|{:^13}|",
                    "Id",
                    "Location",
                    "DateTime",
                    "Duration",
                    "Rating",
                    "Wave Height"
                );
                println!("{:-<80}", "");
                for session in sessions {
                    println!(
                        "|{:^4}|{:^20}|{:^18}|{:^10}|{:^8}|{:^13}|",
                        session.id.map_or_else(
                            || String::from("None"),
                            |x| x.to_string()
                        ),
                        session.location,
                        session.date.format("%Y-%m-%d %H:%M"),
                        format!("{} min", session.duration),
                        format!("{}/10", session.rating),
                        format!("{}", session.wave_height),
                    );
                }
            }
        }
        Command::Config => {
            debug!("Showing configuration...");
            println!("db_path = {}", database_path()?);
        }
    }
    Ok(())
}
