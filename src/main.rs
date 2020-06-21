use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, Error, HttpRequest};
use actix_cors::Cors;
use tokio_postgres::{tls};
use deadpool_postgres::{Pool};
use url::{Url};
use serde::{Serialize, Deserialize};
use tsurezure::dao::*;
use tsurezure::model::*;

#[derive(Clone)]
struct Credential {
    basic_auth: String
}

#[derive(Deserialize, Debug)]
struct CreatePostRequest {
    body: String
}

#[derive(Serialize, Debug)]
struct CreatePostResponse {
    id: Option<i32>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct PostsRequest {
    from: Option<i32>,
    limit: Option<u32>,
}

#[derive(Serialize)]
struct PostsResponse {
    posts: Vec<Post>,
    next: Option<i32>,
}

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
    let posts = posts::find_recent(&*pool.get().await.unwrap(), 5).await.unwrap();
    Ok(web::Json(posts))
}

#[get("/posts")]
async fn get_posts(pool: web::Data<Pool>, web::Query(query): web::Query<PostsRequest>) -> Result<web::Json<PostsResponse>, Error> {
    let limit = query.limit.unwrap_or(5);
    if limit > 50 {
        return Err(actix_web::error::ErrorBadRequest("limit must be <= 50"))
    }
    let client = pool.get().await.unwrap();

    let mut posts = match query.from {
        Some(from_id) => posts::find(&*client, from_id, (limit + 1).into()).await,
        None => posts::find_recent(&*client, (limit + 1).into()).await,
    }.unwrap();
    let next_id = if posts.len() == (limit + 1) as usize {
        Some(posts.last().unwrap().id)
    } else {
        None
    };

    posts.truncate(limit as usize);
    let response = PostsResponse {
        posts: posts,
        next: next_id,
    };
    Ok(web::Json(response))
}

fn authenticate(credential: &Credential, req: &HttpRequest) -> Result<bool, actix_web::error::Error> {
    let auth_header = match req.headers().get("Authorization") {
        Some(val) => Ok(val),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed"))
    }?;

    let tokens: Vec<&str> = auth_header.to_str().unwrap().split(' ').collect();
    match tokens[0] {
        "Basic" => Ok(tokens[1] == credential.basic_auth),
        _ => Err(actix_web::error::ErrorBadRequest("Cannot recognize auth header"))
    }
}

#[post("/posts/new")]
async fn create_post(payload: web::Json<CreatePostRequest>, pool: web::Data<Pool>, credential: web::Data<Credential>, req: HttpRequest) -> Result<web::Json<CreatePostResponse>, Error> {
    let authed = authenticate(&*credential, &req)?;
    if !authed {
        return Err(actix_web::error::ErrorUnauthorized("Invalid auth cred"))
    }

    let post = Post { id: 0, body: payload.body.clone(), posted_at: chrono::Utc::now() };
    let result = posts::save(&*pool.get().await.unwrap(), post).await;
    let response = match result {
        Ok(id) => CreatePostResponse { id: Some(id), error: None },
        Err(err) => CreatePostResponse { id: None, error: Some(format!("{}", err)) }
    };
    Ok(web::Json(response))
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

    let credential = Credential {
        basic_auth: std::env::var("ADMIN_BASIC_AUTH").expect("ADMIN_BASIC_AUTH"),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .finish())
            .data(pool.clone())
            .data(credential.clone())
            .service(index)
            .service(dbtest)
            .service(recent_posts)
            .service(get_posts)
            .service(create_post)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}