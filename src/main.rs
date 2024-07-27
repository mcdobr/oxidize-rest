use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/articles")]
async fn find_articles() -> impl Responder {
    let response = Response {
        message: "Something".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(find_articles))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
