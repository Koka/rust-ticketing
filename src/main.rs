use crate::routes::hello_route;
use actix_web::{App, HttpServer};
use tracing_subscriber::EnvFilter;

mod helpers;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    HttpServer::new(|| App::new().service(hello_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
