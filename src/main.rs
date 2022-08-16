#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use models::NewComment;
use schema::comments::date_posted;
mod models;
mod schema;
use self::{
    models::Comment,
    schema::comments::dsl::{comments, upvotes},
};
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
fn new_comment(comment: Json<NewComment>, conn: CommentDbConn) -> Json<Comment> {
    diesel::insert_into(comments)
        .values(&comment.into_inner())
        .execute(&*conn)
        .expect("Error saving new comment");
    Json(
        comments
            .order(date_posted.desc())
            .limit(1)
            .first(&*conn)
            .expect("Failed to load new comment"),
    )
}

#[post("/comments/upvote/<id>")]
fn upvote_comment(id: u64, conn: CommentDbConn) -> Json<Comment> {
    let rows = diesel::update(comments.find(id))
        .set(upvotes.eq(upvotes + 1))
        .execute(&*conn)
        .expect("Error updating comment");
    assert_eq!(rows, 1, "Expected to update one row");
    Json(
        comments
            .find(id)
            .first::<Comment>(&*conn)
            .expect("Error loading comment"),
    )
}

fn main() {
    rocket::ignite()
        .attach(CommentDbConn::fairing())
        .mount("/api", routes![get_comments, new_comment, upvote_comment])
        .mount("/", StaticFiles::from("./public"))
        .launch();
}
