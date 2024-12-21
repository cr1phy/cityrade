use bcrypt::DEFAULT_COST;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sea_orm::ActiveValue::Set;
use crate::entity;

#[derive(Debug, Default)]
pub struct Account {
    id: Uuid,
    username: String,
    email: String,
    password: Vec<u8>,
    date_of_joining: DateTime<Utc>,
    money: f64,
    diamonds: i64,
}

impl From<entity::account::Model> for Account {
    fn from(model: entity::account::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            password: model.password,
            date_of_joining: model.date_of_joining.and_utc(),
            money: model.money,
            diamonds: model.diamonds,
        }
    }
}

impl From<Account> for entity::account::ActiveModel {
    fn from(account: Account) -> Self {
        Self {
            id: Set(account.id),
            username: Set(account.username),
            email: Set(account.email),
            password: Set(account.password),
            date_of_joining: Set(account.date_of_joining.naive_utc()),
            money: Set(account.money),
            diamonds: Set(account.diamonds),
        }
    }
}

impl Account {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            username,
            email,
            password: bcrypt::hash(password, DEFAULT_COST).expect("Hash failed").into_bytes(),
            date_of_joining: Utc::now(), 
            ..Default::default()
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }

    pub fn set_password(&mut self, password: Vec<u8>) {
        self.password = password;
    }

    pub fn date_of_joining(&self) -> DateTime<Utc> {
        self.date_of_joining
    }

    pub fn money(&self) -> f64 {
        self.money
    }

    pub fn set_money(&mut self, money: f64) {
        self.money = money;
    }

    pub fn diamonds(&self) -> i64 {
        self.diamonds
    }

    pub fn set_diamonds(&mut self, diamonds: i64) {
        self.diamonds = diamonds;
    }
}
