use crate::{api::models::TicketResponse, core::tickets};
use actix_web::{get, HttpResponse, Responder};

#[get("/get-ticket")]
async fn get_ticket() -> impl Responder {
    let ticket_value = tickets::generate_ticket();

    let response_json = &TicketResponse {
        status: "success".to_string(),
        ticket: ticket_value,
    };

    HttpResponse::Ok().json(response_json)
}
