use rocket::FromForm;
use serde::Serialize;

use super::schema::comments;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub body: String,
    pub date_posted: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub username: String,
    pub email: Option<String>,
    pub body: String,
}
