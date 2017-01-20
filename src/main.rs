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

fn main() {
    rocket::ignite()
        .mount("/users/", routes![routes::new_user, routes::get_user])
        .mount("/posts/", routes![routes::new_post, routes::get_post])
        .launch();
}
