#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use models::NewComment;
mod models;
mod schema;
use self::{models::Comment, schema::comments::dsl::*};
use rocket::*;
use rocket_contrib::{
    database, databases::diesel::MysqlConnection, json::Json, serve::StaticFiles,
};

#[database("dialectic")]
struct CommentDbConn(MysqlConnection);

#[get("/comments")]
fn get_comments(conn: CommentDbConn) -> Json<Vec<Comment>> {
    Json(
        comments
            .load::<Comment>(&*conn)
            .expect("Failed to load comments"),
    )
}

#[post("/comments", data = "<comment>")]
fn new_comment(conn: CommentDbConn, comment: Json<NewComment>) {
    diesel::insert_into(comments)
        .values(&comment.0)
        .execute(&*conn)
        .expect("Error saving new comment");
}

fn main() {
    rocket::ignite()
        .attach(CommentDbConn::fairing())
        .mount("/api", routes![get_comments, new_comment])
        .mount("/", StaticFiles::from("./public"))
        .launch();
}
