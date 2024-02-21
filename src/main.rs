use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(HttpResponse::Ok)))
        .bind("0.0.0.0:8000")
        .unwrap()
        .workers(1)
        .run()
        .await
        .unwrap();
}
