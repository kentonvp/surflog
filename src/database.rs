pub(crate) mod models;

/// Get the path to the database file: respects the `XDG_DATA_HOME` environment
/// variable if set, otherwise defaults to `~/.config/surflog.db`.
pub(crate) fn database_path() -> anyhow::Result<String> {
    let loc = std::env::var("XDG_DATA_HOME")
        .unwrap_or(expanduser::expanduser("~")?.display().to_string());
    let path = format!("{}/.config/surflog.db", loc);
    Ok(path)
}

/// Struct representing a connection to the database.
pub(crate) struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    /// Create a new database connection.
    pub(crate) fn new(db_path: &str) -> Database {
        let conn = rusqlite::Connection::open(db_path)
            .expect("failed to open database");

        Database { conn }
    }
}
