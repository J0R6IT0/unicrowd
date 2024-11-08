use serde::Serialize;

#[derive(Serialize)]
pub struct TicketResponse {
    pub status: String,
    pub ticket: String,
}
