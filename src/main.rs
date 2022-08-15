#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use models::NewComment;
mod models;
mod schema;
use self::{models::Comment, schema::comments::dsl::*};
use rocket::{request::Form, *};
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
fn new_comment(comment: Form<NewComment>, conn: CommentDbConn) {
    diesel::insert_into(comments)
        .values(&comment.into_inner())
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
