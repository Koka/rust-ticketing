use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::helpers::init;

mod helpers;
mod migrations;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    init::setup_tracing();

    let config = helpers::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    init::run_migrations(pool.clone()).await.unwrap();

    init::run_server(config, pool).await
}
