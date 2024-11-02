use actix_web::Responder;

pub async fn handle_request() -> impl Responder {
    "Handler response"
}

