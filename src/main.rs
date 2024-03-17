use actix_web::{web, App, HttpServer, Responder};
mod models;
mod controller;

#[actix_web::main]
async fn main() {
    let addr = "localhost:8080";
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(controller::index))
    })
        .bind(addr)
        .unwrap()
        .run();
    println!("Server live at http://{}", addr);
    server.await.unwrap();
}