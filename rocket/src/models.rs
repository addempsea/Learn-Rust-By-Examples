use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::posts;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String
}