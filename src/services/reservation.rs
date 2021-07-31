use deadpool_postgres::Pool;
use eyre::Result;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ReservedTicket {
    id: i64,
    price_cents: u32,
}

pub async fn reserve_tickets(
    db_pool: Arc<Pool>,
    ticket_type_id: i32,
    buyer_id: i64,
    ticket_count: usize,
) -> Result<Vec<ReservedTicket>> {
    let db_pool = db_pool.clone();
    let mut client = db_pool.get().await?;

    let tx = client.transaction().await?;

    let mut tickets: Vec<ReservedTicket> = Vec::with_capacity(ticket_count);

    for _ in 0..ticket_count {
        let result = tx
            .query(
                include_str!("sql/reserve_ticket.sql"),
                &[&ticket_type_id, &buyer_id],
            )
            .await?;

        result
            .iter()
            .map(|row| {
                let id: i64 = row.get(0);
                let price_cents: i32 = row.get(1);
                ReservedTicket {
                    id,
                    price_cents: price_cents as u32,
                }
            })
            .for_each(|id| tickets.push(id));
    }

    tx.commit().await?;

    Ok(tickets)
}
