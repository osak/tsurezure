use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, Error};
use tokio_postgres::{connect, Client, tls};
use deadpool_postgres::{ClientWrapper, Pool};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/dbtest")]
async fn dbtest(pool: web::Data<Pool>) -> Result<String, Error> {
    let client = pool.get().await.unwrap();
    let rows = client.query("SELECT * FROM test", &[]).await.unwrap();
    Ok(rows[0].get("msg"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u32 = std::env::var("PORT").unwrap().parse().unwrap();

    let mut cfg = deadpool_postgres::Config::default();
    cfg.user = Some("tsurezure".to_owned());
    cfg.password = Some("tsurezure".to_owned());
    cfg.dbname = Some("tsurezure".to_owned());
    cfg.host = Some("db".to_owned());
    cfg.port = Some(5432);

    let pool = cfg.create_pool(tls::NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(dbtest)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}