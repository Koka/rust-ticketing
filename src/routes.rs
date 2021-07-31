use crate::helpers::actix::wrap_err;
use crate::services::hello::hello;
use actix_web::web::Data;
use actix_web::{get, Result};
use deadpool_postgres::Pool;

#[get("/")]
pub async fn hello_route(db_pool: Data<Pool>) -> Result<String> {
    wrap_err(hello(db_pool.into_inner()).await)
}
