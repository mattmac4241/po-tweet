extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;

use models::users::{User, NewUser, get_by_id, get_by_username};
use routes::routes::*;

#[post("/", data="<new_user>", format="application/json")]
pub fn new_user(new_user: JSON<NewUser>) -> JSON<ResponseMessage> {
    let mut user = new_user;
    if user.insert() {
        create_response_message("Ok".to_string(), None)
    } else {
        create_response_message("error".to_string(), Some("Failed to add user.".to_string()))
    }
}

#[get("/<id>")]
fn get_user_by_id(id: i32) -> JSON<ModelMessage<User>>  {
    let user = get_by_id(id);
    match user {
        Ok(u) => {
            create_model_message(Some(u), None)
        }
        Err(e) => {
            create_model_message(None, Some(e.to_string()))
        }
    }
}

#[get("/<name>", rank=2)]
fn get_user_by_username(name: String) -> JSON<ModelMessage<User>>  {
    let user = get_by_username(name);
    match user {
        Ok(u) => {
            create_model_message(Some(u), None)
        }
        Err(e) => {
            create_model_message(None, Some(e.to_string()))
        }
    }
}
