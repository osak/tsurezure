use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, Error};
use tokio_postgres::{tls};
use deadpool_postgres::{Pool};
use url::{Url};
use tsurezure::dao::posts::*;
use tsurezure::model::*;

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

#[get("/posts/recent")]
async fn recent_posts(pool: web::Data<Pool>) -> Result<web::Json<Vec<Post>>, Error> {
    let posts = find_recent(&*pool.get().await.unwrap(), 5).await.unwrap();
    Ok(web::Json(posts))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u32 = std::env::var("PORT").unwrap().parse().unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let url = Url::parse(&db_url).expect("Url::parse");
    let mut path_segments = url.path_segments().expect("path segments");

    let mut cfg = deadpool_postgres::Config::default();
    cfg.user = Some(url.username().to_owned());
    cfg.password = Some(url.password().unwrap().to_owned());
    cfg.dbname = Some(path_segments.next().unwrap().to_owned());
    cfg.host = Some(url.host_str().unwrap().to_owned());
    cfg.port = Some(url.port().unwrap());

    let pool = cfg.create_pool(tls::NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(dbtest)
            .service(recent_posts)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}