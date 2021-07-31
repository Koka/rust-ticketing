use crate::helpers::actix::wrap_err;
use crate::services::hello::hello;
use actix_web::{get, Result};

#[get("/")]
pub async fn hello_route() -> Result<&'static str> {
    wrap_err(hello().await)
}
