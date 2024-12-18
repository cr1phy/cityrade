mod routes;

use std::io;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().configure(routes::init))
        .bind_auto_h2c("0.0.0.0:8888")?
        .run()
        .await?;
    Ok(())
}
