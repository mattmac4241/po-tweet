#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket;

mod routes;
mod models;
mod database;
mod schema;

use rocket_contrib::JSON;
use models::User;
use std::collections::HashMap;

type SimpleMap = HashMap<&'static str, &'static str>;

#[post("/", data="<user>", format="application/json")]
pub fn new(user: JSON<User>) -> JSON<SimpleMap> {
    if user.insert() {
        JSON(map!{ "status" => "ok" })
    } else {
        JSON(map!{
                    "status" => "error",
                    "reason" => "Failed to add user."
                })
    }
}

fn main() {
    rocket::ignite().mount("/users", routes![new]).launch()
}
