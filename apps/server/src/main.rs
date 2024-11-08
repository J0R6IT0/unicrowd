use actix_web::{middleware::Logger, App, HttpServer};
use api::endpoints::{get_ticket, validate_ticket};
use core::database;
use std::io::Result;

pub mod api;
pub mod core;
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    database::init_db().await;

    HttpServer::new(move || {
        App::new()
            .service(get_ticket)
            .service(validate_ticket)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
