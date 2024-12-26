use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the role of an account in the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
    Moderator,
}

/// Represents an account in the system.
///
/// # Fields
///
/// * `id` - A unique identifier for the account.
/// * `username` - The username associated with the account.
/// * `email` - The email address associated with the account.
/// * `password_hash` - A hash of the account's password.
/// * `date_of_registration` - The date and time when the account was registered.
/// * `money` - The amount of money the account has.
/// * `diamonds` - The number of diamonds the account has.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    id: Uuid,
    username: String,
    email: String,
    password_hash: Vec<u8>,
    date_of_registration: DateTime<Utc>,
    money: f64,
    diamonds: i64,
    role: Role,
}

impl Account {
    pub fn new(id: Uuid, username: String, email: String, password_hash: Vec<u8>, role: Role) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
            date_of_registration: Utc::now(),
            money: 0.0,
            diamonds: 0,
            role,
        }
    }
}
