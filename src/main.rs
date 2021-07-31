use crate::helpers::config::Config;
use crate::routes::ping::ping_route;
use crate::routes::reservation::reserve_ticket_route;
use crate::routes::users::sign_up_route;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use deadpool_postgres::Pool;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use tracing_actix_web::TracingLogger;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod helpers;
mod migrations;
mod routes;
mod services;

async fn run_migrations(db_pool: Pool) -> eyre::Result<()> {
    let mut obj = db_pool.get().await?;
    let conn = &mut **obj;
    migrations::runner().run_async(conn).await?;
    Ok(())
}

fn setup_tracing() {
    color_eyre::install().unwrap();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(ErrorLayer::default())
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    setup_tracing();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    run_migrations(pool.clone()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(Data::new(pool.clone()))
            .service(ping_route)
            .service(reserve_ticket_route)
            .service(sign_up_route)
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
