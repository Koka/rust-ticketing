use crate::helpers::actix::wrap_err;
use crate::services::reservation::reserve_tickets;
use crate::services::reservation::ReservedTicket;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::{post, Result};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReserveRequest {
    ticket_type_id: i32,
    buyer_id: i64,
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
) -> Result<HttpResponse> {
    let ReserveRequest {
        ticket_type_id,
        buyer_id,
        ticket_count,
    } = *data;
    let tickets = wrap_err(
        reserve_tickets(
            db_pool.into_inner(),
            ticket_type_id,
            buyer_id,
            if ticket_count == 0 { 1 } else { ticket_count },
        )
        .await,
    )?;

    Ok(HttpResponse::Ok().json(ReserveResponse { tickets }))
}
