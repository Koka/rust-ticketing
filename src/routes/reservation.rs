use crate::helpers::error::ActixResult;
use crate::helpers::jwt::UserId;
use crate::services::reservation::reserve_tickets;
use crate::services::reservation::ReservedTicket;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::{post};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReserveRequest {
    ticket_type_id: i32,
    #[serde(default)]
    ticket_count: usize,
}

#[derive(Serialize)]
pub struct ReserveResponse {
    tickets: Vec<ReservedTicket>,
}

#[post("/reserve")]
pub async fn reserve_ticket_route(
    db_pool: Data<Pool>,
    data: Json<ReserveRequest>,
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    let ext = req.extensions();
    let user_id: Option<&UserId> = ext.get();
    let buyer_id = user_id.unwrap().0;

    let ReserveRequest {
        ticket_type_id,
        ticket_count,
    } = *data;

    let tickets = reserve_tickets(
        db_pool.into_inner(),
        ticket_type_id,
        buyer_id,
        if ticket_count == 0 { 1 } else { ticket_count },
    )
    .await?;

    Ok(HttpResponse::Ok().json(ReserveResponse { tickets }))
}
