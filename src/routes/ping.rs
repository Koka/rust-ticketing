use crate::helpers::error::ActixResult;
use crate::services::ping::ping;
use actix_web::get;
use actix_web::web::Data;
use deadpool_postgres::Pool;

#[get("/ping")]
pub async fn ping_route(db_pool: Data<Pool>) -> ActixResult<String> {
    Ok(ping(db_pool.into_inner()).await?)
}
