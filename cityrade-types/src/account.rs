use crate::resources::Resource;
use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub resources: Resource,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(
        id: String,
        username: String,
        password_hash: String,
        resources: Resource,
    ) -> Account {
        Account {
            id,
            username,
            password_hash,
            resources,
            created_at: Utc::now(),
            last_login: None,
        }
    }

    pub fn generate_jwt(&self) -> String {
        let claims = Claims {
            sub: self.id.clone(),
            exp: (Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(std::env::var("KEY").unwrap().as_bytes()),
        )
        .unwrap();
        token
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
    }

    pub fn check_password(&self, password: &str) -> bool {
        password == self.password_hash
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
