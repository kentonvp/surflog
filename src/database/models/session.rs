use chrono::NaiveDateTime;

use crate::database::Database;
use log::debug;

#[derive(Debug)]
/// Struct representing a surf session.
pub(crate) struct Session {
    pub(crate) id: Option<u32>,
    pub(crate) location: String,
    pub(crate) date: NaiveDateTime,
    pub(crate) duration: u16,
    pub(crate) rating: u8,
    pub(crate) wave_height: f32,
}

/// SQL statement to create the sessions table if it does not exist.
pub(crate) const CREATE_TABLE_STR: &str = {
    r#"CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location TEXT NOT NULL,
    datetime DATETIME NOT NULL,
    duration INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    wave_height REAL NOT NULL
)"#
};

/// Builder pattern
pub(crate) struct SessionBuilder {
    location: Option<String>,
    date: Option<NaiveDateTime>,
    duration: Option<u16>,
    rating: Option<u8>,
    wave_height: Option<f32>,
}

impl SessionBuilder {
    pub(crate) fn builder() -> Self {
        SessionBuilder {
            location: None,
            date: None,
            duration: None,
            rating: None,
            wave_height: None,
        }
    }

    pub(crate) fn location(&mut self, location: String) {
        self.location = Some(location);
    }

    pub(crate) fn date(&mut self, date: NaiveDateTime) {
        self.date = Some(date);
    }

    pub(crate) fn duration(&mut self, duration: u16) {
        self.duration = Some(duration);
    }

    pub(crate) fn rating(&mut self, rating: u8) {
        self.rating = Some(rating);
    }

    pub(crate) fn wave_height(&mut self, wave_height: f32) {
        self.wave_height = Some(wave_height);
    }

    pub(crate) fn build(self) -> anyhow::Result<Session> {
        Ok(Session {
            id: None,
            location: self
                .location
                .ok_or_else(|| anyhow::anyhow!("location is required"))?,
            date: self
                .date
                .ok_or_else(|| anyhow::anyhow!("date is required"))?,
            duration: self
                .duration
                .ok_or_else(|| anyhow::anyhow!("duration is required"))?,
            rating: self
                .rating
                .ok_or_else(|| anyhow::anyhow!("rating is required"))?,
            wave_height: self
                .wave_height
                .ok_or_else(|| anyhow::anyhow!("wave_height is required"))?,
        })
    }
}

/// Helper utility to parse a row from the session table into a `Session` struct.
pub(crate) fn parse(
    row: &rusqlite::Row,
) -> anyhow::Result<Session, rusqlite::Error> {
    let date_str: String = row.get(2)?;
    let date =
        chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
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
pub(crate) fn insert(db: &Database, session: &Session) -> anyhow::Result<()> {
    debug!("Inserting session into database: {:?}", session);
    let query = "INSERT INTO sessions (location, datetime, duration, rating, wave_height) VALUES (?1, ?2, ?3, ?4, ?5)";
    db.conn
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

/// Get all sessions for a particular location.
pub(crate) fn get_by_location(
    db: &Database,
    location: &str,
) -> anyhow::Result<Vec<Session>> {
    debug!("Getting sessions for location: {}", location);
    let mut stmt = db
        .conn
        .prepare("SELECT id, location, datetime, duration, rating, wave_height FROM sessions WHERE location=?1 COLLATE NOCASE")?;
    let mut rows = stmt.query_map([location], parse)?;
    let mut sessions = Vec::new();
    while let Some(Ok(row)) = rows.next() {
        sessions.push(row);
    }

    Ok(sessions)
}

/// Get all sessions in the database.
pub(crate) fn get_all(db: &Database) -> anyhow::Result<Vec<Session>> {
    debug!("Getting all sessions from database");
    let mut stmt = db.conn.prepare(
        "SELECT id, location, datetime, duration, rating, wave_height FROM sessions",
    )?;
    let mut rows = stmt.query_map([], parse)?;
    let mut sessions = Vec::new();
    while let Some(Ok(row)) = rows.next() {
        sessions.push(row);
    }

    Ok(sessions)
}

/// Delete a session by its ID.
pub(crate) fn delete(db: &Database, id: u64) {
    debug!("Deleting session with id: {}", id);
    db.conn
        .execute("DELETE FROM sessions WHERE id = ?1", [id])
        .expect("failed to delete session");
}

pub(crate) fn get_last_location(db: &Database) -> Option<String> {
    debug!("Getting last session location from database");
    let mut stmt = db
        .conn
        .prepare(
            "SELECT location FROM sessions ORDER BY datetime DESC LIMIT 1",
        )
        .ok()?;
    let mut rows = stmt.query_map([], |row| row.get(0)).ok()?;
    rows.next().and_then(|res| res.ok())
}
