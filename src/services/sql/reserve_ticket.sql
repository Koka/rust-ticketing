INSERT INTO tickets (ticket_type_id, buyer_id) VALUES ($1, $2) RETURNING id, price_cents
