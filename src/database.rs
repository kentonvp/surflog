use anyhow::Result;
use chrono::NaiveDateTime;
use expanduser::expanduser;
use rusqlite::Connection;

// Re-exports
pub use models::Session;

mod models {
    use chrono::NaiveDateTime;

    #[derive(Debug)]
    /// Struct representing a surf session.
    pub struct Session {
        pub id: Option<u32>,
        pub location: String,
        pub date: NaiveDateTime,
        pub duration: u16,
        pub rating: u8,
        pub wave_height: f32,
    }
}

/// Get the path to the databaes file: respects the `XDG_DATA_HOME` environment variable if set,
/// otherwise defaults to `~/.config/surflog.db`.
pub fn database_path() -> Result<String> {
    let loc = std::env::var("XDG_DATA_HOME")
        .unwrap_or(expanduser("~")?.display().to_string());
    let path = format!("{}/.config/surflog.db", loc);
    Ok(path)
}

/// Struct representing a connection to the database.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection.
    pub fn new(db_path: &str) -> Database {
        let conn = Connection::open(db_path).expect("failed to open database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                location TEXT NOT NULL,
                datetime DATETIME NOT NULL,
                duration INTEGER NOT NULL,
                rating INTEGER NOT NULL,
                wave_height REAL NOT NULL
            )",
            (),
        )
        .expect("failed to create sessions table");

        Database { conn }
    }

    /// Helper utility to parse a row from the session table into a `Session` struct.
    pub fn parse_session(
        row: &rusqlite::Row,
    ) -> Result<Session, rusqlite::Error> {
        let date_str: String = row.get(2)?;
        let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
            .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                2,
                rusqlite::types::Type::Text,
                Box::new(e),
            )
        })?;
        Ok(Session {
            id: row.get(0)?,
            location: row.get(1)?,
            date,
            duration: row.get(3)?,
            rating: row.get(4)?,
            wave_height: row.get(5)?,
        })
    }

    /// Insert a new session into the database session table.
    pub fn insert_session(&self, session: &Session) -> Result<()> {
        let query = "INSERT INTO sessions (location, datetime,duration, rating, wave_height) VALUES (?1, ?2, ?3, ?4, ?5)";
        self.conn
            .execute(
                query,
                (
                    &session.location,
                    &session.date.format("%Y-%m-%d %H:%M").to_string(),
                    &session.duration,
                    &session.rating,
                    &session.wave_height,
                ),
            )
            .expect("failed to insert query");

        Ok(())
    }

    /// Get a session by its ID.
    pub fn get_sessions_by_location(
        &self,
        location: &str,
    ) -> Result<Vec<Session>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, location, datetime, duration, rating, wave_height FROM sessions WHERE location=?1 COLLATE NOCASE")?;
        let mut rows = stmt.query_map([location], Self::parse_session)?;
        let mut sessions = Vec::new();
        while let Some(Ok(row)) = rows.next() {
            sessions.push(row);
        }

        Ok(sessions)
    }

    /// Get all sessions in the database.
    pub fn get_sessions(&self) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, location, datetime, duration, rating, wave_height FROM sessions",
        )?;
        let mut rows = stmt.query_map([], Self::parse_session)?;
        let mut sessions = Vec::new();
        while let Some(Ok(row)) = rows.next() {
            sessions.push(row);
        }

        Ok(sessions)
    }

    /// Delete a session by its ID.
    pub fn delete_session(&self, id: u64) {
        self.conn
            .execute("DELETE FROM sessions WHERE id = ?1", [id])
            .expect("failed to delete session");
    }
}
