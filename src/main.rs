/*
 ________       ___  ___      ________      ________      ___           ________      ________      ________
 |\   ____\     |\  \|\  \    |\   __  \    |\  _____\    |\  \         |\   __  \    |\   ____\    |\   ____\
 \ \  \___|_    \ \  \\\  \   \ \  \|\  \   \ \  \__/     \ \  \        \ \  \|\  \   \ \  \___|    \ \  \___|_
 \ \_____  \    \ \  \\\  \   \ \   _  _\   \ \   __\     \ \  \        \ \  \\\  \   \ \  \  ___   \ \_____  \
  \|____|\  \    \ \  \\\  \   \ \  \\  \|   \ \  \_|      \ \  \____    \ \  \\\  \   \ \  \|\  \   \|____|\  \
    ____\_\  \    \ \_______\   \ \__\\ _\    \ \__\        \ \_______\   \ \_______\   \ \_______\    ____\_\  \
   |\_________\    \|_______|    \|__|\|__|    \|__|         \|_______|    \|_______|    \|_______|   |\_________\
   \|_________|                                                                                       \|_________|
*/

use chrono::NaiveDateTime;

mod cli;
mod database;

use database::{database_path, Database, Session};

use anyhow::Result;
use log::debug;

fn main() -> Result<()> {
    let args = cli::get_args();

    let db = Database::new(&database_path()?);

    match args.cmd {
        cli::Command::Add(add_opts) => {
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
        cli::Command::Delete(del_opts) => {
            debug!("Delete a session...");
            db.delete_session(del_opts.id);
        }
        cli::Command::List(list_opts) => {
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
        cli::Command::Config => {
            debug!("Showing configuration...");
            println!("db_path = {}", database_path()?);
        }
    }
    Ok(())
}
