use crate::config::Config;
use crate::routes::hello_route;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use tracing_subscriber::EnvFilter;

mod config;
mod helpers;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    color_eyre::install().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(hello_route)
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
