#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
use rocket::{self, routes};


pub mod routes;
pub mod cors;
pub mod models;
pub mod schema;

use routes::DbConn;


fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing) 
        .mount("/", routes![routes::hello, routes::hello_name])
        .mount("/notes", 
            routes!
                [
                    routes::get_note, 
                    routes::get_notes, 
                    routes::create_note, 
                    routes::edit_note, 
                    routes::publish_note
                ]
        )
    .launch();
}
