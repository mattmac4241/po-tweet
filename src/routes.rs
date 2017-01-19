extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;
use models::{User, NewUser, get_user_by_id};
use std::collections::HashMap;

type SimpleMap = HashMap<&'static str, &'static str>;

#[derive(Serialize)]
struct UserMessage {
    user: Option<User>,
    error: String,
}

#[post("/", data="<new_user>", format="application/json")]
pub fn new(new_user: JSON<NewUser>) -> JSON<SimpleMap> {
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
fn get(id: i32) -> JSON<UserMessage>  {
    let user = get_user_by_id(id);
    match user {
        Ok(u) => {
            let message = UserMessage{
                user: Some(u),
                error: "".to_string(),
            };
            JSON(message)
        }
        Err(e) => {
            let message = UserMessage{
                user: None,
                error: format!("{}", e),
            };
            JSON(message)
        }
    }
}
