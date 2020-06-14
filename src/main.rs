use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u32 = env::var("PORT").unwrap().parse().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}