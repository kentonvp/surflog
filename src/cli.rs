use clap::{Args, Parser, Subcommand};

/// A simple command-line tool to track surf sessions.
#[derive(Parser)]
#[command(author="Kenton Van Peursem", version="0.0.1", about="CLI tool to track surf sessions", long_about = None)]
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
