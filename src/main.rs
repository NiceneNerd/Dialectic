#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
// use flume::{bounded, unbounded, Receiver, Sender};
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
    tokio::{
        select,
        sync::broadcast::{channel, error::RecvError, Sender},
    },
    Shutdown, *,
};
use rocket_sync_db_pools::database;

#[database("dialectic")]
struct CommentDbConn(MysqlConnection);

#[derive(rocket::serde::Serialize, Debug, Copy, Clone)]
struct UpvoteUpdate {
    id: u64,
    upvotes: i32,
}

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
async fn upvote_comment(
    id: u64,
    db: CommentDbConn,
    ctx: &State<Sender<UpvoteUpdate>>,
) -> Json<Comment> {
    let res = db
        .run(move |conn| {
            let rows = diesel::update(comments.find(id))
                .set(upvotes.eq(upvotes + 1))
                .execute(&*conn)
                .expect("Error updating comment");
            assert_eq!(rows, 1, "Expected to update one row");
            let comment = comments
                .find(id)
                .first::<Comment>(&*conn)
                .expect("Error loading comment");
            Json(comment)
        })
        .await;
    let _ = ctx.send(UpvoteUpdate {
        id,
        upvotes: res.upvotes,
    });
    res
}

#[get("/upvotes")]
async fn stream(ctx: &State<Sender<UpvoteUpdate>>, mut end: Shutdown) -> EventStream![Event + '_] {
    let mut recv = ctx.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = recv.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CommentDbConn::fairing())
        .manage(channel::<UpvoteUpdate>(1024).0)
        .mount(
            "/api",
            routes![get_comments, new_comment, upvote_comment, stream],
        )
        .mount("/", FileServer::from("./dist"))
}
