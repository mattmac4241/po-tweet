#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate bcrypt;

mod routes;
mod models;
mod database;
mod schema;

use routes::posts::*;
use routes::users::*;

fn main() {
    rocket::ignite()
        .mount("/users/", routes![new_user, get_user_by_id, get_user_by_username])
        .mount("/posts/", routes![new_post, get_post, delete_post])
        .launch();
}
