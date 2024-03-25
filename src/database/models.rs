use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Session {
    pub id: Option<i32>,
    pub location: String,
    pub date: NaiveDateTime,
    pub duration: u16,
    pub rating: u8,
    pub wave_height: f32,
}
