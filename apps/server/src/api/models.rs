use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TicketResponse {
    pub status: String,
    pub ticket: String,
}

#[derive(Deserialize)]
pub struct ValidateTicketRequest {
    #[serde(default)]
    pub ticket: String,
}

#[derive(Serialize)]
pub struct ValidateTicketResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
