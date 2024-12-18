use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: Vec<u8>,
    pub date_of_joining: DateTime<Utc>,
    pub money: f64,
    pub diamonds: i64,
}