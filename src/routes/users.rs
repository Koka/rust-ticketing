use crate::helpers::actix::wrap_err;
use crate::helpers::config::Config;
use crate::helpers::jwt::issue_token;
use crate::services::users::create_user;

use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::{post, Result};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignUpRequest {
    name: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    user_id: i64,
    jwt_token: String,
}

#[post("/users")]
pub async fn sign_up_route(
    db_pool: Data<Pool>,
    config: Data<Config>,
    data: Json<SignUpRequest>,
) -> Result<HttpResponse> {
    let SignUpRequest { name } = &*data;

    let user_id = wrap_err(create_user(db_pool.into_inner(), name).await)?;
    let jwt_token = wrap_err(issue_token(config.jwt_secret.as_bytes(), user_id).await)?;
    Ok(HttpResponse::Ok().json(SignUpResponse { user_id, jwt_token }))
}
