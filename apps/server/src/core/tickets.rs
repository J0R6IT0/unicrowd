use crate::{core::database, models::Ticket};

pub fn generate_ticket() -> String {
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
    ticket.insert(&mut conn).unwrap();

    ticket_value
}
