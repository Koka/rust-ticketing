use crate::helpers::actix::wrap_err;
use crate::services::ping::ping;
use actix_web::web::Data;
use actix_web::{get, Result};
use deadpool_postgres::Pool;

#[get("/ping")]
pub async fn ping_route(db_pool: Data<Pool>) -> Result<String> {
    wrap_err(ping(db_pool.into_inner()).await)
}
