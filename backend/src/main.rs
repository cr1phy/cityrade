mod auth;
mod database;
mod entity;
mod routes;
mod types;

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
    Migrator::up(&db.conn, None).await.unwrap();

    let state = AppState { db };

    HttpServer::new(move || App::new().app_data(state.clone()).configure(routes::init))
        .bind_auto_h2c(&server_url)?
        .run()
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    mod routes {
        use actix_web::{test, web, App};
        use serde_json::json;
        use crate::{routes, AppState, database::Database};

        #[actix_web::test]
        async fn test_status() {
            let app = test::init_service(App::new().configure(routes::init)).await;
            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["status"], "Ok!");
        }

        #[actix_web::test]
        async fn test_signup() {
            let state = web::Data::new(AppState {
                db: Database::new_mock().await,
            });

            let app =
                test::init_service(App::new().app_data(state.clone()).configure(routes::init))
                    .await;

            let req_data = json!({
                "username": "testuser",
                "email": "test@example.com",
                "password": "secure123"
            });

            let req = test::TestRequest::post()
                .uri("/auth/signup")
                .set_json(&req_data)
                .to_request();

            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["message"], "Account created");
        }

        #[actix_web::test]
        async fn test_login_successful() {
            let state = web::Data::new(AppState {
                db: Database::new_mock().await,
            });

            let password = bcrypt::hash("secure123", bcrypt::DEFAULT_COST).unwrap();

            state
                .db
                .save_account(crate::entity::account::Model {
                    id: uuid::Uuid::now_v7(),
                    username: "testuser".to_string(),
                    email: "test@example.com".to_string(),
                    password: password.into_bytes(),
                    date_of_joining: chrono::Utc::now().naive_utc(),
                    money: 0.0,
                    diamonds: 0,
                })
                .await
                .unwrap();

            let app =
                test::init_service(App::new().app_data(state.clone()).configure(routes::init))
                    .await;

            let req_data = json!({
                "email": "test@example.com",
                "password": "secure123"
            });

            let req = test::TestRequest::post()
                .uri("/auth/login")
                .set_json(&req_data)
                .to_request();

            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["message"], "Login successful");
            assert!(body["token"].as_str().is_some());
        }

        #[actix_web::test]
        async fn test_login_unsuccessful() {
            let app = test::init_service(App::new().configure(routes::init)).await;

            let req_data = json!({
                "email": "invalid@example.com",
                "password": "wrongpassword"
            });

            let req = test::TestRequest::post()
                .uri("/auth/login")
                .set_json(&req_data)
                .to_request();

            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["error"], "Invalid email or password");
        }

        #[actix_web::test]
        async fn test_logout() {
            let app = test::init_service(App::new().configure(routes::init)).await;

            let req = test::TestRequest::post().uri("/auth/logout").to_request();
            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["message"], "Logout successful");
        }

        #[actix_web::test]
        async fn test_twofa() {
            let app = test::init_service(App::new().configure(routes::init)).await;

            let req_data = json!({
                "email": "test@example.com",
                "code": "123456"
            });

            let req = test::TestRequest::post()
                .uri("/auth/ota")
                .set_json(&req_data)
                .to_request();

            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

            let body: serde_json::Value = test::read_body_json(resp).await;
            assert_eq!(body["error"], "Invalid code");
        }
    }
}
