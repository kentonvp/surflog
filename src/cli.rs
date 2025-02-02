use anyhow::Result;
use chrono::NaiveDateTime;
use clap::{Args, Parser, Subcommand};
use log::debug;

use super::database::{database_path, Database, Session};

/// A simple command-line tool to track surf sessions.
#[derive(Parser)]
#[command(
    author="Kenton Van Peursem",
    version="0.0.1",
    about="CLI tool to track surf sessions",
    long_about = None
)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
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
pub struct Delete {
    /// `id` of the surf session to delete
    pub id: u64,
}

#[derive(Args, Debug)]
pub struct List {
    /// Optional location of surf sessions to list
    pub location: Option<String>,
}

#[derive(Args, Debug)]
pub struct Add {
    /// The surf spot
    #[arg(required = true)]
    pub location: String,

    /// The date and time of the surf session in format "YYYY-mm-dd HH:MM"
    #[arg(required = true)]
    pub datetime: String,

    /// The time surfed in minutes
    #[arg(required = true)]
    pub duration: u16,

    /// The rating for the surf session out of 10
    #[arg(required = true)]
    pub rating: u8,

    /// The wave height in feet
    #[arg(required = true)]
    pub wave_height: f32,
}

pub fn get_args() -> Cli {
    Cli::parse()
}

pub fn run() -> Result<()> {
    let args = get_args();

    let db = Database::new(&database_path()?);

    match args.cmd {
        Command::Add(add_opts) => {
            debug!("Adding a new session...");
            let session = Session {
                id: None,
                location: add_opts.location,
                date: NaiveDateTime::parse_from_str(
                    &add_opts.datetime,
                    "%Y-%m-%d %H:%M",
                )
                .unwrap(),
                duration: add_opts.duration,
                rating: add_opts.rating,
                wave_height: add_opts.wave_height,
            };

            // Insert session
            let _ = db.insert_session(&session);
        }
        Command::Delete(del_opts) => {
            debug!("Delete a session...");
            db.delete_session(del_opts.id);
        }
        Command::List(list_opts) => {
            debug!("Listing all sessions...");
            // Display all sessions
            let sessions = match list_opts.location {
                Some(loc) => db.get_sessions_by_location(&loc),
                None => db.get_sessions(),
            };

            if let Ok(sessions) = sessions {
                //println!("{:-<80}", "");
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
                //println!("{:-<80}", "");
            }
        }
        Command::Config => {
            debug!("Showing configuration...");
            println!("db_path = {}", database_path()?);
        }
    }
    Ok(())
}
