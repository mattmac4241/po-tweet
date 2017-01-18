// #![feature(plugin)]
// #![plugin(rocket_codegen)]
// extern crate rocket;
// extern crate serde_json;
//
//
// use std::io;
// use std::path::Path;
//
// use rocket::Data;
//
// use rocket_contrib::JSON;
// use models::User;
// use std::collections::HashMap;
//
// type SimpleMap = HashMap<&'static str, &'static str>;
//
// #[post("/", data="<user>", format="application/json")]
// pub fn new(user: JSON<User>) -> JSON<SimpleMap> {
//     if user.insert() {
//         JSON(map!{ "status" => "ok" })
//     } else {
//         JSON(map!{
//                     "status" => "error",
//                     "reason" => "Failed to add user."
//                 })
//     }
// }
