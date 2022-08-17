#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use flume::{bounded, unbounded, Receiver, Sender};
use models::NewComment;
use schema::comments::date_posted;
mod models;
mod schema;
use self::{
    models::Comment,
    schema::comments::dsl::{comments, upvotes},
};
use rocket::{
    fs::FileServer,
    response::stream::{Event, EventStream},
    serde::json::Json,
    Shutdown, *,
};
use rocket_sync_db_pools::database;

#[database("dialectic")]
struct CommentDbConn(MysqlConnection);

#[derive(rocket::serde::Serialize, Debug)]
struct UpvoteUpdate {
    id: u64,
    upvotes: i32,
}

#[derive(Clone)]
struct UpvoteSender(Sender<UpvoteUpdate>);
struct UpvoteReceiver(Receiver<UpvoteUpdate>);

#[get("/comments")]
async fn get_comments(db: CommentDbConn) -> Json<Vec<Comment>> {
    db.run(move |conn| {
        Json(
            comments
                .load::<Comment>(&*conn)
                .expect("Failed to load comments"),
        )
    })
    .await
}

#[post("/comments", data = "<comment>")]
async fn new_comment(comment: Json<NewComment>, db: CommentDbConn) -> Json<Comment> {
    db.run(move |conn| {
        diesel::insert_into(comments)
            .values(&comment.into_inner())
            .execute(conn)
            .expect("Error saving new comment");
        Json(
            comments
                .order(date_posted.desc())
                .limit(1)
                .first(&*conn)
                .expect("Failed to load new comment"),
        )
    })
    .await
}

#[post("/comments/upvote/<id>")]
async fn upvote_comment(id: u64, db: CommentDbConn, ctx: &State<UpvoteSender>) -> Json<Comment> {
    let ctx = ctx.0.clone();
    db.run(move |conn| {
        let rows = diesel::update(comments.find(id))
            .set(upvotes.eq(upvotes + 1))
            .execute(&*conn)
            .expect("Error updating comment");
        assert_eq!(rows, 1, "Expected to update one row");
        let comment = comments
            .find(id)
            .first::<Comment>(&*conn)
            .expect("Error loading comment");
        ctx.send(UpvoteUpdate {
            id,
            upvotes: comment.upvotes,
        })
        .expect("Failed to send upvote notification");
        Json(comment)
    })
    .await
}

#[get("/upvotes")]
async fn stream(ctx: &State<UpvoteReceiver>, mut shutdown: Shutdown) -> EventStream![Event + '_] {
    EventStream! {
        loop {
            if let Ok(upvote) = ctx.0.recv_async().await {
                println!("upvote: {:?}", upvote);
                yield Event::json(&upvote);
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    let (send, recv): (Sender<UpvoteUpdate>, Receiver<UpvoteUpdate>) = unbounded();
    rocket::build()
        .attach(CommentDbConn::fairing())
        .manage(UpvoteSender(send))
        .manage(UpvoteReceiver(recv))
        .mount(
            "/api",
            routes![get_comments, new_comment, upvote_comment, stream],
        )
        .mount("/", FileServer::from("./dist"))
}
