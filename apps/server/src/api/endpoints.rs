use super::models::ValidateTicketRequest;
use crate::{
    api::models::{TicketResponse, ValidateTicketResponse},
    core::tickets,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

const TEMP_SECRET: &str = "secret";

#[get("/get-ticket")]
async fn get_ticket() -> impl Responder {
    let ticket_value = tickets::generate_ticket();

    if ticket_value.is_err() {
        return HttpResponse::InternalServerError().json(&TicketResponse {
            status: "error".to_string(),
            ticket: ticket_value.err().unwrap().to_string(),
        });
    }

    let response_json = &TicketResponse {
        status: "success".to_string(),
        ticket: ticket_value.unwrap(),
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/validate-ticket")]
async fn validate_ticket(
    req: HttpRequest,
    body: web::Form<ValidateTicketRequest>,
) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap_or(""),
        None => "",
    };

    if auth_header.is_empty() {
        return HttpResponse::Unauthorized().json(&ValidateTicketResponse {
            status: "error".to_string(),
            error: Some("Authorization header is missing".to_string()),
        });
    }

    if auth_header != TEMP_SECRET {
        return HttpResponse::Unauthorized().json(&ValidateTicketResponse {
            status: "error".to_string(),
            error: Some("Unauthorized access".to_string()),
        });
    }

    let ticket = &body.ticket;
    if ticket.is_empty() {
        return HttpResponse::BadRequest().json(&ValidateTicketResponse {
            status: "error".to_string(),
            error: Some("Missing 'ticket' field".to_string()),
        });
    }

    let is_valid = tickets::validate_ticket(ticket);

    if is_valid {
        HttpResponse::Ok().json(&ValidateTicketResponse {
            status: "success".to_string(),
            error: None,
        })
    } else {
        HttpResponse::Unauthorized().json(&ValidateTicketResponse {
            status: "error".to_string(),
            error: Some("Invalid ticket".to_string()),
        })
    }
}
