#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::web::routes;

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().configure(routes::config_routes)).await;
        let req = test::TestRequest::get().uri("/").send_request(&app).await;
        assert!(req.status().is_success());
    }
}

