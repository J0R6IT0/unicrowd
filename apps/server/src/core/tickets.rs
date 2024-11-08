use crate::{core::database, models::Ticket};

const TEMP_LIMIT: u32 = 5;

pub fn generate_ticket() -> Result<String, &'static str> {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let ticket_value: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let ticket = Ticket {
        id: None,
        ticket: ticket_value.clone(),
        valid: true,
    };

    let mut conn = database::create_connection();

    let count = Ticket::count(&mut conn).unwrap();

    if count >= TEMP_LIMIT as i64 {
        return Err("Ticket limit reached");
    }

    ticket.insert(&mut conn).unwrap();

    Ok(ticket_value)
}

pub fn validate_ticket(ticket: &str) -> bool {
    let mut conn = database::create_connection();
    let mut ticket = Ticket::select(&mut conn, ticket);

    if let Ok(ref mut ticket) = ticket {
        if ticket.valid {
            ticket.valid = false;
            ticket.update(&mut conn).unwrap();
            return true;
        }
    }
    false
}
