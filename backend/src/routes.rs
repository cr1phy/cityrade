use actix_web::{get, post, web::ServiceConfig, HttpResponse};
use serde_json::json;

#[get("/")]
async fn status() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "Ok!", "version": env!("CARGO_PKG_VERSION")}))
}

// TODO: Регистрация аккаунта
#[post("/auth/signup")]
async fn signup() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO: Вход в аккаунт
#[post("/auth/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO: Выход из аккаунта
#[post("/auth/logout")]
async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO: Двухфакторная аутентификация по e-mail.
#[post("/auth/ota")]
async fn twofa() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(status)
        .service(signup)
        .service(login)
        .service(logout)
        .service(twofa);
}
