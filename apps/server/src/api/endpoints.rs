use crate::api::models::GenericResponse;
use actix_web::{get, HttpResponse, Responder};

#[get("/get-ticket")]
async fn get_ticket() -> impl Responder {
    const MESSAGE: &str = "Hello, World!";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}
