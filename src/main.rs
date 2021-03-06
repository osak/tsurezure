use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, get, post, put, Error, dev::ServiceRequest};
use actix_cors::Cors;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web_httpauth::{middleware::HttpAuthentication, extractors::basic::BasicAuth};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use tsurezure::model::*;
use tsurezure::view;
use tsurezure::schema;

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

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/api/posts/recent")]
async fn recent_posts(pool: web::Data<DbPool>) -> Result<web::Json<Vec<view::Post>>, Error> {
    use schema::posts::dsl::*;

    let raw_posts = posts.order(id.desc()).limit(5).load::<Post>(&pool.get().unwrap()).unwrap();
    let view_posts: Vec<view::Post> = raw_posts.into_iter()
      .map(|p| p.into())
      .collect();
    Ok(web::Json(view_posts))
}

#[get("/api/posts")]
async fn get_posts(pool: web::Data<DbPool>, web::Query(query): web::Query<PostsRequest>) -> Result<web::Json<PostsResponse>, Error> {
    use schema::posts::dsl::*;

    let limit = query.limit.unwrap_or(5);
    if limit > 50 {
        return Err(actix_web::error::ErrorBadRequest("limit must be <= 50"))
    }
    let client = pool.get().unwrap();

    let mut raw_posts = match query.from {
        Some(from_id) => posts.filter(id.le(from_id)).order(id.desc()).load::<Post>(&client),
        None => posts.order(id.desc()).limit((limit + 1) as i64).load::<Post>(&*client),
    }.unwrap();
    let next_id = if raw_posts.len() == (limit + 1) as usize {
        Some(raw_posts.last().unwrap().id)
    } else {
        None
    };

    raw_posts.truncate(limit as usize);
    let view_posts: Vec<view::Post> = raw_posts.into_iter()
      .map(|p| p.into())
      .collect();
    let response = PostsResponse {
        posts: view_posts,
        next: next_id,
    };
    Ok(web::Json(response))
}

#[get("/api/posts/{id}")]
async fn admin_get_post(pool: web::Data<DbPool>, info: web::Path<AdminGetPostInfo>, id: Identity) -> Result<web::Json<AdminGetPostResponse>, actix_web::Error> {
    use schema::posts::dsl::posts;

    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        _ => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let client = pool.get().unwrap();
    let post = posts.find(info.id).load::<Post>(&client).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
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
async fn create_post(payload: web::Form<CreatePostRequest>, pool: web::Data<DbPool>, id: Identity) -> Result<web::Json<CreatePostResponse>, Error> {
    use schema::posts::dsl::posts;

    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let post = NewPost { body: payload.body.to_owned(), posted_at: chrono::Utc::now() };
    let result = diesel::insert_into(posts)
        .values(&post)
        .get_result::<Post>(&pool.get().unwrap());
    let response = match result {
        Ok(p) => CreatePostResponse { id: Some(p.id), error: None },
        Err(err) => CreatePostResponse { id: None, error: Some(format!("{}", err)) }
    };
    Ok(web::Json(response))
}

#[put("/api/posts/{id}")]
async fn update_post(payload: web::Json<UpdatePostRequest>, pool: web::Data<DbPool>, id: Identity) -> Result<web::Json<CreatePostResponse>, Error> {
    use schema::posts::dsl::{self, posts};

    match id.identity() {
        Some(name) if name == "admin" => Ok(()),
        Some(_) => Err(actix_web::error::ErrorForbidden("Admin only".to_owned())),
        None => Err(actix_web::error::ErrorUnauthorized("Auth needed".to_owned()))
    }?;

    let client = pool.get().unwrap();
    
    let result = diesel::update(posts.find(payload.id))
        .set((
            dsl::body.eq(&payload.body),
            dsl::updated_at.eq(chrono::Utc::now())
        ))
        .get_result::<Post>(&client);

    result.map(|r| web::Json(CreatePostResponse { id: Some(r.id), error: None }))
        .map_err(|_| actix_web::error::ErrorNotFound("Not found"))
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

#[get("/debug/diesel_test")]
async fn diesel_test(pool: web::Data<DbPool>) -> Result<web::Json<Vec<view::Post>>, Error> {
    use schema::posts::dsl::*;

    let results = posts.filter(schema::posts::id.gt(1))
        .limit(5)
        .load::<Post>(&*pool.get().unwrap())
        .expect("load");
    Ok(web::Json(results.into_iter().map(|i| i.into()).collect()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u32 = std::env::var("PORT").unwrap().parse().unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let cookie_key = std::env::var("COOKIE_KEY").expect("COOKIE_KEY");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let diesel_pool = r2d2::Pool::builder()
      .build(manager)
      .expect("pool");

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
            .data(diesel_pool.clone())
            .service(recent_posts)
            .service(get_posts)
            .service(create_post_page)
            .service(diesel_test)
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