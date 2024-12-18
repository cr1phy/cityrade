
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

use crate::entity::{prelude::*, account};

#[derive(Debug, Clone)]
pub struct Database {
    pub conn: DatabaseConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let conn = sea_orm::Database::connect(database_url).await?;
        Ok(Self { conn })
    }

    pub async fn save_account(&self, account: account::Model) -> Result<(), DbErr> {
        let active_account = account.into_active_model();
        active_account.insert(&self.conn).await?;
        Ok(())
    }

    pub async fn get_account_by_email(&self, email: &str) -> Result<account::Model, DbErr> {
        let account = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.conn)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("No account with email: {}", email)))?;
        Ok(account)
    }
}

