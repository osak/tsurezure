#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::postgres;

#[database("db")]
struct DbConn(postgres::Connection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/dbtest")]
fn dbtest(conn: DbConn) -> String {
    conn.0.query("SELECT msg FROM test", &[]).unwrap()
        .into_iter()
        .next()
        .unwrap()
        .get("msg")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, dbtest])
        .attach(DbConn::fairing())
        .launch();
}
