use actix_web::{web, App, HttpServer, Responder};
//use config;
//use db::repository::CommandRepository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Command {
    id: i32,
    command: String,
}

async fn index() -> impl Responder {
    "Welcome to Cortex: Your command-line and web utility!"
}

async fn get_commands(repo: web::Data<CommandRepository>) -> impl Responder {
    match repo.get_commands() {
        Ok(commands) => web::Json(commands),
        Err(_) => web::Json(vec![]),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    let repo = CommandRepository::new(&config.db_path).expect("Database initialization failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .route("/", web::get().to(index))
            .route("/commands", web::get().to(get_commands))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
