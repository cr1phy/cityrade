use sea_orm::{
    entity::*,
    prelude::*,
    DatabaseBackend,
    MockDatabase,
    MockExecResult,
};
use uuid::Uuid;
use crate::entity::account;

#[derive(Debug, Clone)]
pub struct Database {
    pub conn: DbConn,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let conn = sea_orm::Database::connect(database_url).await?;
        Ok(Self { conn })
    }

    pub async fn new_mock() -> Self {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                vec![account::Model {
                    id: Uuid::now_v7(),
                    username: "mock_user".to_string(),
                    email: "mock_user@example.com".to_string(),
                    password: b"hashed_password".to_vec(),
                    date_of_joining: chrono::Utc::now().naive_utc(),
                    money: 100.0,
                    diamonds: 50,
                }],
            ])
            .append_exec_results(vec![
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
            ])
            .into_connection();

        Self { conn: connection }
    }

    pub async fn save_account(&self, new_account: account::Model) -> Result<(), DbErr> {
        account::ActiveModel {
            id: ActiveValue::Set(new_account.id),
            username: ActiveValue::Set(new_account.username),
            email: ActiveValue::Set(new_account.email),
            password: ActiveValue::Set(new_account.password),
            date_of_joining: ActiveValue::Set(new_account.date_of_joining),
            money: ActiveValue::Set(new_account.money),
            diamonds: ActiveValue::Set(new_account.diamonds),
        }
        .insert(&self.conn)
        .await?;
        Ok(())
    }

    pub async fn get_account_by_email(&self, email: &str) -> Result<account::Model, DbErr> {
        account::Entity::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.conn)
            .await?
            .ok_or(DbErr::Custom("Account not found".to_string()))
    }
}
