extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;

use models::posts::{Post, NewPost, get_post_by_id};
use routes::routes::*;


#[post("/", data="<new_post>", format="application/json")]
pub fn new_post(new_post: JSON<NewPost>) -> JSON<ResponseMessage> {
    if new_post.insert() {
        create_response_message("Ok".to_string(), None)
    } else {
        create_response_message("Error".to_string(), Some("Failed to add post".to_string()))
    }
}

#[get("/<id>")]
fn get_post(id: i32) -> JSON<ModelMessage<Post>>  {
    let post = get_post_by_id(id);
    match post {
        Ok(p) => {
            create_model_message(Some(p), None)
        }
        Err(e) => {
            create_model_message(None, Some(e.to_string()))
        }
    }
}

#[delete("/<id>")]
fn delete_post(id: i32) -> JSON<ResponseMessage> {
    let post = get_post_by_id(id);
    match post {
        Ok(p) => {
            if p.delete() {
                create_response_message("Ok".to_string(), None)
            } else {
                create_response_message("Error".to_string(), Some("Failed to delete post".to_string()))
            }
        }
        Err(e) => {
            create_response_message("Error".to_string(), Some(e.to_string()))
        }
    }
}
