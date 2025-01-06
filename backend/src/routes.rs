use actix_web::{get, web::ServiceConfig};

#[get("/")]
async fn status() -> String {
    "Server is running".into()
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(status);
}