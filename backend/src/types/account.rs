use chrono::{DateTime, Utc};
use uuid::Uuid;
use sea_orm::ActiveValue::Set;
use crate::entity;

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
