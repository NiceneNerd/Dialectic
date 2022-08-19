#![allow(clippy::extra_unused_lifetimes)]
use super::schema::comments;
use rocket::{
    serde::{Deserialize, Serialize},
    FromForm,
};

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
#[serde(crate = "rocket::serde")]
pub struct ResComment {
    pub id: u64,
    pub children: Vec<ResComment>,
    pub name: String,
    pub body: String,
    pub upvotes: i32,
    pub date_posted: chrono::NaiveDateTime,
}

#[derive(Deserialize, FromForm, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "comments"]
pub struct NewComment {
    pub name: String,
    pub body: String,
    pub parent_id: Option<u64>,
}
