#![feature(decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
// use flume::{bounded, unbounded, Receiver, Sender};
use models::{NewComment, ResComment};
use schema::comments::date_posted;
mod models;
mod schema;
use self::{
    models::Comment,
    schema::comments::dsl::{comments, parent_id, upvotes},
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

impl ResComment {
    pub fn from_comment(comment: Comment, conn: &mut MysqlConnection) -> ResComment {
        let children = comments
            .filter(parent_id.eq(comment.id))
            .load::<Comment>(conn)
            .unwrap()
            .into_iter()
            .map(|c| ResComment::from_comment(c, conn))
            .collect();
        ResComment {
            id: comment.id,
            children,
            name: comment.name,
            body: comment.body,
            upvotes: comment.upvotes,
            date_posted: comment.date_posted,
        }
    }
}

#[derive(rocket::serde::Serialize, Debug, Copy, Clone)]
struct UpvoteUpdate {
    id: u64,
    upvotes: i32,
}

#[get("/comments")]
async fn get_comments(db: CommentDbConn) -> Json<Vec<ResComment>> {
    db.run(move |conn| {
        Json(
            comments
                .filter(parent_id.is_null())
                .load::<Comment>(&*conn)
                .expect("Failed to load comments")
                .into_iter()
                .map(|c| ResComment::from_comment(c, &mut *conn))
                .collect(),
        )
    })
    .await
}

#[post("/comments", data = "<comment>")]
async fn new_comment(comment: Json<NewComment>, db: CommentDbConn) -> Json<ResComment> {
    db.run(move |conn| {
        diesel::insert_into(comments)
            .values(&comment.into_inner())
            .execute(conn)
            .expect("Error saving new comment");
        Json(ResComment::from_comment(
            comments
                .order(date_posted.desc())
                .limit(1)
                .first(&*conn)
                .expect("Failed to load new comment"),
            conn,
        ))
    })
    .await
}

#[post("/comments/upvote/<id>")]
async fn upvote_comment(
    id: u64,
    db: CommentDbConn,
    ctx: &State<Sender<UpvoteUpdate>>,
) -> Json<ResComment> {
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
            Json(ResComment::from_comment(comment, conn))
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
