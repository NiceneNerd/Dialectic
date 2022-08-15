#![feature(decl_macro)]
use rocket::*;
use rocket_contrib::{database, databases::diesel::MysqlConnection, serve::StaticFiles};

#[database("dialectic")]
struct CommentDb(MysqlConnection);

#[get("/comments")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(CommentDb::fairing())
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("./public"))
        .launch();
}
