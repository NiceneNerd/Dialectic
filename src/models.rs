use rocket::FromForm;
use serde::Serialize;

use super::schema::comments;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: u64,
    pub name: String,
    pub body: String,
    pub upvotes: i32,
    pub date_posted: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, FromForm, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub name: String,
    #[column_name = "body"]
    pub comment: String,
}
