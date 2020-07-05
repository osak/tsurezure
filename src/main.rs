use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, get, post, put, Error, dev::ServiceRequest};
use actix_cors::Cors;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
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

#[derive(Deserialize)]
struct AdminGetPostInfo {
    id: i32,
}

#[derive(Serialize)]
struct AdminGetPostResponse {
    post: view::RawPost
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    id: i32,
    body: String,
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

#[get("/api/posts/{id}")]
async fn admin_get_post(pool: web::Data<Pool>, info: web::Path<AdminGetPostInfo>, id: Identity) -> Result<web::Json<AdminGetPostResponse>, actix_web::Error> {
    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        _ => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let client = pool.get().await.unwrap();
    let post = posts::find(&*client, info.id, 1).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    if post.len() > 0 {
        let raw_post: view::RawPost = post[0].clone().into();
        Ok(web::Json(AdminGetPostResponse{post: raw_post}))
    } else {
        Err(actix_web::error::ErrorNotFound("Not found"))
    }
}

#[get("/posts/new")]
async fn create_post_page(id: Identity) -> Result<fs::NamedFile, Error> {
    match id.identity() {
        Some(name) if name == "admin" => fs::NamedFile::open("asset/post.html").map_err(|e| actix_web::error::ErrorInternalServerError(e)),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        _ => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }
}

#[post("/posts/new")]
async fn create_post(payload: web::Form<CreatePostRequest>, pool: web::Data<Pool>, id: Identity) -> Result<web::Json<CreatePostResponse>, Error> {
    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let post = Post { id: 0, body: payload.body.to_owned(), posted_at: chrono::Utc::now(), updated_at: None };
    let result = posts::save(&*pool.get().await.unwrap(), post).await;
    let response = match result {
        Ok(id) => CreatePostResponse { id: Some(id), error: None },
        Err(err) => CreatePostResponse { id: None, error: Some(format!("{}", err)) }
    };
    Ok(web::Json(response))
}

#[put("/api/posts/{id}")]
async fn update_post(payload: web::Json<UpdatePostRequest>, pool: web::Data<Pool>, id: Identity) -> Result<web::Json<CreatePostResponse>, Error> {
    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let client = pool.get().await.unwrap();
    let posts = posts::find(&*client, payload.id, 1).await?;
    if posts.len() > 0 {
        let post = &posts[0];
        let new_post = Post {
            id: payload.id,
            body: payload.body.to_owned(),
            posted_at: post.posted_at,
            updated_at: Some(chrono::Utc::now()),
        };
        posts::update(&*client, &new_post)
            .await
            .map(|id| web::Json(CreatePostResponse { id: Some(id), error: None }))
            .map_err(|e| e.into())
    } else {
        Err(actix_web::error::ErrorNotFound("Not found"))
    }
}

async fn default_route(req: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    use actix_web::http::header::{HeaderName, HeaderValue};

    match req.path() {
        "/index.bundle.js" => fs::NamedFile::open("web-dist/index.bundle.js").map(|n| n.into_response(&req).unwrap()),
        "/admin.bundle.js" => fs::NamedFile::open("web-dist/admin.bundle.js").map(|n| n.into_response(&req).unwrap()),
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

async fn default_admin_route(req: HttpRequest, id: Identity) -> Result<HttpResponse, Error> {
    use actix_web::http::header::{HeaderName, HeaderValue};

    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;


    match req.path() {
        "/style.css" => fs::NamedFile::open("web-dist/style.css")
            .map(|n| n.into_response(&req).unwrap())
            .map_err(|e| actix_web::error::ErrorInternalServerError(e)),
        _ => fs::NamedFile::open("web-dist/admin.html")
              .map(|n| {
                  let mut resp = n.into_response(&req).unwrap();
                  let headers = resp.headers_mut();
                  headers.append(HeaderName::from_static("cache-control"), HeaderValue::from_static("no-cache"));
                  resp
              })
              .map_err(|e| actix_web::error::ErrorInternalServerError(e))
    }
}

async fn validator(req: ServiceRequest, cred: BasicAuth) -> Result<ServiceRequest, Error> {
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

#[get("")]
async fn login(id: Identity) -> HttpResponse {
    id.remember("admin".to_owned());
    HttpResponse::Ok().finish()
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

    let cookie_key = std::env::var("COOKIE_KEY").expect("COOKIE_KEY");

    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validator);
        let identity = IdentityService::new(
            CookieIdentityPolicy::new(&cookie_key.as_bytes())
                .name("auth")
                .secure(false));
        let cors = Cors::new().supports_credentials().finish();
        App::new()
            .wrap(cors)
            .wrap(identity)
            .data(pool.clone())
            .service(recent_posts)
            .service(get_posts)
            .service(create_post_page)
            .service(web::scope("/login")
                .wrap(auth)
                .service(login))
            .service(web::scope("/admin")
                .service(admin_get_post)
                .service(create_post)
                .service(update_post)
                .default_service(web::resource("").route(web::get().to(default_admin_route))))
            .default_service(web::resource("").route(web::get().to(default_route)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}