use chrono::NaiveDateTime;
use homedir::get_my_home;
use rusqlite::{Connection, Result};

pub mod models;

use models::Session;

pub fn database_path() -> String {
    let home = get_my_home()
        .expect("failed to get home directory")
        .expect("failed to get home directory");
    format!(
        "{}/.config/surflog.db",
        home.to_str()
            .expect("failed to convert home directory to string")
    )
}

pub struct Database {
    conn: Connection,
}

impl Database {
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

    pub fn parse_session(row: &rusqlite::Row) -> Result<Session, rusqlite::Error> {
        let date_str: String = row.get(2)?;
        let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M").map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
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

    pub fn get_sessions_by_location(&self, location: &str) -> Result<Vec<Session>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, location, datetime, duration, rating, wave_height FROM sessions WHERE location=?1")?;
        let mut rows = stmt.query_map([location], Self::parse_session)?;
        let mut sessions = Vec::new();
        while let Some(Ok(row)) = rows.next() {
            sessions.push(row);
        }

        Ok(sessions)
    }

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

    pub fn delete_session(&self, id: u64) {
        self.conn
            .execute("DELETE FROM sessions WHERE id = ?1", [id])
            .expect("failed to delete session");
    }
}
