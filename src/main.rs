use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, get, post, Error, dev::ServiceRequest};
use actix_cors::Cors;
use actix_web_httpauth::{middleware::HttpAuthentication, extractors::basic::BasicAuth};
use tokio_postgres::{tls};
use deadpool_postgres::{Pool};
use url::{Url};
use serde::{Serialize, Deserialize};
use tsurezure::dao::*;
use tsurezure::model::*;
use tsurezure::view;

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
    posts: Vec<view::Post>,
    next: Option<i32>,
}

#[get("/api/posts/recent")]
async fn recent_posts(pool: web::Data<Pool>) -> Result<web::Json<Vec<view::Post>>, Error> {
    let raw_posts = posts::find_recent(&*pool.get().await.unwrap(), 5).await.unwrap();
    let view_posts: Vec<view::Post> = raw_posts.into_iter()
      .map(|p| p.into())
      .collect();
    Ok(web::Json(view_posts))
}

#[get("/api/posts")]
async fn get_posts(pool: web::Data<Pool>, web::Query(query): web::Query<PostsRequest>) -> Result<web::Json<PostsResponse>, Error> {
    let limit = query.limit.unwrap_or(5);
    if limit > 50 {
        return Err(actix_web::error::ErrorBadRequest("limit must be <= 50"))
    }
    let client = pool.get().await.unwrap();

    let mut posts = match query.from {
        Some(from_id) => posts::find(&*client, from_id, limit + 1).await,
        None => posts::find_recent(&*client, limit + 1).await,
    }.unwrap();
    let next_id = if posts.len() == (limit + 1) as usize {
        Some(posts.last().unwrap().id)
    } else {
        None
    };

    posts.truncate(limit as usize);
    let view_posts: Vec<view::Post> = posts.into_iter()
      .map(|p| p.into())
      .collect();
    let response = PostsResponse {
        posts: view_posts,
        next: next_id,
    };
    Ok(web::Json(response))
}

#[get("/posts/new")]
async fn create_post_page() -> Result<fs::NamedFile, std::io::Error> {
    fs::NamedFile::open("asset/post.html")
}

#[post("/posts/new")]
async fn create_post(payload: web::Form<CreatePostRequest>, pool: web::Data<Pool>) -> Result<web::Json<CreatePostResponse>, Error> {
    let post = Post { id: 0, body: payload.body.to_owned(), posted_at: chrono::Utc::now() };
    let result = posts::save(&*pool.get().await.unwrap(), post).await;
    let response = match result {
        Ok(id) => CreatePostResponse { id: Some(id), error: None },
        Err(err) => CreatePostResponse { id: None, error: Some(format!("{}", err)) }
    };
    Ok(web::Json(response))
}

async fn default_route(req: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    use actix_web::http::header::{HeaderName, HeaderValue};

    match req.path() {
        "/bundle.js" => fs::NamedFile::open("web-dist/bundle.js").map(|n| n.into_response(&req).unwrap()),
        "/style.css" => fs::NamedFile::open("web-dist/style.css").map(|n| n.into_response(&req).unwrap()),
        _ => fs::NamedFile::open("web-dist/index.html")
              .map(|n| {
                  let mut resp = n.into_response(&req).unwrap();
                  let headers = resp.headers_mut();
                  headers.append(HeaderName::from_static("cache-control"), HeaderValue::from_static("no-cache"));
                  resp
              })
    }
}

async fn validator(req: ServiceRequest, cred: BasicAuth) -> Result<ServiceRequest, Error> {
    let path = req.path();
    if !path.starts_with("/admin") {
        return Ok(req)
    }

    let admin_user = std::env::var("ADMIN_USER").expect("ADMIN_USER");
    let admin_pass = std::env::var("ADMIN_PASS").expect("ADMIN_PASS");
    let cred_user = cred.user_id();
    let cred_pass = cred.password().unwrap();
    if *cred_user == admin_user && *cred_pass == admin_pass {
        Ok(req)
    } else {
        Err(actix_web::error::ErrorUnauthorized("Auth failed"))
    }
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
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(
                Cors::new()
                    .finish())
            .data(pool.clone())
            .service(recent_posts)
            .service(get_posts)
            .service(web::scope("/admin")
                .wrap(auth)
                .service(create_post_page)
                .service(create_post))
            .default_service(web::resource("").route(web::get().to(default_route)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}