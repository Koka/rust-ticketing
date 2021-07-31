use crate::helpers::config::Config;
use crate::routes::ping::ping_route;
use crate::routes::reservation::reserve_ticket_route;
use crate::routes::users::sign_up_route;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use tracing_subscriber::EnvFilter;

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
            .service(ping_route)
            .service(reserve_ticket_route)
            .service(sign_up_route)
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
