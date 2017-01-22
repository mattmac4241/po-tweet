use rocket_contrib::JSON;

use models::users::{User, get_by_id, get_by_username};
use routes::routes::*;

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
