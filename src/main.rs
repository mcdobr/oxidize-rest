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

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use crate::find_articles;

    #[actix_web::test]
    async fn should_find_articles() {
        let app = test::init_service(App::new().service(find_articles)).await;
        let request = test::TestRequest::get().uri("/articles").to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());
    }
}
