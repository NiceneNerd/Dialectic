#![allow(clippy::extra_unused_lifetimes)]
use rocket::FromForm;
use serde::Serialize;

use super::schema::comments;

#[derive(Queryable, Debug)]
pub struct Comment {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub body: String,
    pub upvotes: i32,
    pub date_posted: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct ResComment {
    pub id: u64,
    pub children: Vec<ResComment>,
    pub name: String,
    pub body: String,
    pub upvotes: i32,
    pub date_posted: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, FromForm, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub name: String,
    pub body: String,
    pub parent_id: Option<u64>,
}
