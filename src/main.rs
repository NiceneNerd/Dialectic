#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use models::NewComment;
mod models;
mod schema;
use self::{
    models::Comment,
    schema::comments::dsl::{comments, upvotes},
};
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
fn new_comment(comment: Form<NewComment>, conn: CommentDbConn) -> response::Redirect {
    diesel::insert_into(comments)
        .values(&comment.into_inner())
        .execute(&*conn)
        .expect("Error saving new comment");
    response::Redirect::to("/")
}

#[put("/comments/upvote/<id>")]
fn upvote_comment(id: u64, conn: CommentDbConn) {
    let rows = diesel::update(comments.find(id))
        .set(upvotes.eq(upvotes + 1))
        .execute(&*conn)
        .expect("Error updating comment");
}

fn main() {
    rocket::ignite()
        .attach(CommentDbConn::fairing())
        .mount("/api", routes![get_comments, new_comment, upvote_comment])
        .mount("/", StaticFiles::from("./public"))
        .launch();
}
