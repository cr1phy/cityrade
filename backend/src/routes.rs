use actix_web::{get, post, web, web::ServiceConfig, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::{entity::account, AppState};

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct OtaRequest {
    pub email: String,
    pub code: String,
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "Ok!", "version": env!("CARGO_PKG_VERSION")}))
}

#[post("/auth/signup")]
async fn signup(state: web::Data<AppState>, data: web::Json<SignupRequest>) -> impl Responder {
    let db = &state.db;

    if data.username.is_empty() || data.email.is_empty() || data.password.len() < 6 {
        return HttpResponse::BadRequest().json(json!({"error": "Invalid signup data"}));
    }

    let new_account = account::Model {
        id: Uuid::now_v7(),
        username: data.username.clone(),
        email: data.email.clone(),
        password: bcrypt::hash(&data.password, bcrypt::DEFAULT_COST).expect("Hash failed").into_bytes(),
        date_of_joining: chrono::Utc::now().naive_utc(),
        money: 0.0,
        diamonds: 0,
    };

    match db.save_account(new_account).await {
        Ok(_) => HttpResponse::Created().json(json!({"message": "Account created"})),
        Err(e) => {
            eprintln!("Error saving account: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Signup failed"}))
        }
    }
}

#[post("/auth/login")]
async fn login(state: web::Data<AppState>, data: web::Json<LoginRequest>) -> impl Responder {
    let db = &state.db;

    let account = match db.get_account_by_email(&data.email).await {
        Ok(acc) => acc,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({"error": "Invalid email or password"}));
        }
    };

    if !bcrypt::verify(&data.password, &String::from_utf8(account.password.clone()).unwrap()).unwrap_or(false) {
        return HttpResponse::Unauthorized().json(json!({"error": "Invalid email or password"}));
    }

    let token = crate::auth::create_jwt(account.id).unwrap_or_else(|_| "".to_string());

    HttpResponse::Ok().json(json!({"message": "Login successful", "token": token}))
}

#[post("/auth/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Logout successful"}))
}

#[post("/auth/ota")]
async fn twofa(data: web::Json<OtaRequest>) -> impl Responder {
    match crate::auth::validate_ota_code(&data.email, &data.code).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "2FA success"})),
        Err(_) => HttpResponse::Unauthorized().json(json!({"error": "Invalid code"})),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(status)
        .service(signup)
        .service(login)
        .service(logout)
        .service(twofa);
}
