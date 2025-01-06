mod routes;

use std::{env, io};
use actix_web::{App, HttpServer};
use sea_orm::{Database, DbConn};

#[derive(Debug, Clone)]
struct AppState {
    _db: DbConn,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // TODO: auth for accounts
    // let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let db = Database::connect(&db_url).await.unwrap();
    // TODO: migrator

    let state = AppState { _db: db };

    HttpServer::new(move || App::new().app_data(state.clone()).configure(routes::init))
        .bind_auto_h2c(&server_url)?
        .run()
        .await?;
    Ok(())
}