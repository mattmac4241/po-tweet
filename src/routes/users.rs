extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;

use models::users::{User, NewUser, get_user_by_id};

use std::collections::HashMap;

pub type SimpleMap = HashMap<&'static str, &'static str>;

#[derive(Serialize)]
pub struct Message<T> {
    model: Option<T>,
    error: String,
}

#[post("/", data="<new_user>", format="application/json")]
pub fn new_user(new_user: JSON<NewUser>) -> JSON<SimpleMap> {
    let mut user = new_user;
    if user.insert() {
        JSON(map!{ "status" => "ok" })
    } else {
        JSON(map!{
                    "status" => "error",
                    "reason" => "Failed to add user."
                })
    }
}

#[get("/<id>")]
fn get_user(id: i32) -> JSON<Message<User>>  {
    let user = get_user_by_id(id);
    match user {
        Ok(u) => {
            let message = Message{
                model: Some(u),
                error: "".to_string(),
            };
            JSON(message)
        }
        Err(e) => {
            let message = Message{
                model: None,
                error: format!("{}", e),
            };
            JSON(message)
        }
    }
}
