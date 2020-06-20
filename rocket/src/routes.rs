use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket::{self, get, post, put};
use diesel::prelude::*;

use crate::models::{Post, NewPost};
use crate::schema::posts;

#[database("postgres")]
pub struct DbConn(PgConnection);

#[get("/")]
pub fn get_notes(conn: DbConn) -> Json<Vec<Post>> {
    let notes = posts::table
        .order(posts::columns::id.desc())
        .load::<Post>(&*conn)
        .unwrap();
    Json(notes)
}

#[get("/<id>")]
pub fn get_note(conn: DbConn,  id: i32) -> Json<Vec<Post>> {
    let note = posts::table
        .filter(posts::columns::id.eq(id))
        .load::<Post>(&*conn)
        .unwrap();
    Json(note)
}

#[post("/", data = "<new_note>")]
pub fn create_note(conn: DbConn, new_note: Json<NewPost>) -> Json<Post> {
    let result = diesel::insert_into(posts::table)
        .values(&*new_note)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[put("/<id>", data = "<new_note>")]
pub fn edit_note(conn: DbConn, id: i32, new_note: Json<NewPost>) -> Json<Post> {
    let note = posts::table
               .filter(posts::columns::id.eq(id));
    let edited_note = diesel::update(note)
               .set((posts::columns::title.eq(&new_note.title), posts::columns::body.eq(&new_note.body)))
               .get_result(&*conn)
               .unwrap();
    Json(edited_note)              
}

#[put("/publish/<id>")]
pub fn publish_note(conn: DbConn, id: i32) -> Json<Post> {
    let note = posts::table
               .filter(posts::columns::id.eq(id));
    let edited_note = diesel::update(note)
               .set(posts::columns::published.eq(true))
               .get_result(&*conn)
               .unwrap();
    Json(edited_note)              
}


#[get("/")]
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/<name>")]
pub fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}