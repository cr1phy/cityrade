mod auth;
mod database;
mod entity;
mod routes;
mod types;
#[cfg(test)]
mod tests;

use std::{env, io};

use crate::database::Database;
use actix_web::{App, HttpServer};
use migration::{Migrator, MigratorTrait};

#[derive(Debug, Clone)]
struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let db = Database::new(&db_url).await.unwrap();
    Migrator::up(db.conn.as_ref(), None).await.unwrap();

    let state = AppState { db };

    HttpServer::new(move || App::new().app_data(state.clone()).configure(routes::init))
        .bind_auto_h2c(&server_url)?
        .run()
        .await?;
    Ok(())
}