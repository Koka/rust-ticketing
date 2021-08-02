use crate::helpers::config::Config;
use crate::helpers::jwt::token_validator;
use crate::routes::ping::ping_route;
use crate::routes::reservation::reserve_ticket_route;
use crate::routes::users::sign_up_route;
use actix_cors::Cors;
use actix_web::web::scope;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use deadpool_postgres::Pool;
use tracing_actix_web::TracingLogger;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub async fn run_migrations(db_pool: Pool) -> eyre::Result<()> {
    let mut obj = db_pool.get().await?;
    let conn = &mut **obj;
    crate::migrations::runner().run_async(conn).await?;
    Ok(())
}

pub fn setup_tracing() {
    color_eyre::install().unwrap();
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(ErrorLayer::default())
        .init();
}

pub async fn run_server(config: Config, pool: Pool) -> std::io::Result<()> {
    let server_addr = config.server_addr.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            .service(ping_route)
            .service(sign_up_route)
            .service(
                scope("/")
                    .wrap(HttpAuthentication::bearer(token_validator))
                    .service(reserve_ticket_route),
            )
    })
    .bind(server_addr)?
    .run()
    .await
}
