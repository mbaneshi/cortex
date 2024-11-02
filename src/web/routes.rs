use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(super::handlers::handle_request)));
}
