#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde_json;


use std::io;
use std::path::Path;

use rocket::Data;

use rocket_contrib::JSON;
use models::{User, NewUser, get_user_by_id};
use std::collections::HashMap;

type SimpleMap = HashMap<&'static str, &'static str>;

#[post("/", data="<new_user>", format="application/json")]
pub fn new(new_user: JSON<NewUser>) -> JSON<SimpleMap> {
    if new_user.insert() {
        JSON(map!{ "status" => "ok" })
    } else {
        JSON(map!{
                    "status" => "error",
                    "reason" => "Failed to add user."
                })
    }
}

#[get("/<id>")]
fn get(id: i32) -> Option<JSON<User>>  {
    let user = get_user_by_id(id);
    Some(JSON(user.unwrap()))
}
