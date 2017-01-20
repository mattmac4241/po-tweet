extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;

use models::posts::{Post, NewPost, get_post_by_id};

use std::collections::HashMap;

pub type SimpleMap = HashMap<&'static str, &'static str>;

#[derive(Serialize)]
pub struct Message<T> {
    model: Option<T>,
    error: String,
}

#[post("/", data="<new_post>", format="application/json")]
pub fn new_post(new_post: JSON<NewPost>) -> JSON<SimpleMap> {
    if new_post.insert() {
        JSON(map!{ "status" => "ok" })
    } else {
        JSON(map!{
                    "status" => "error",
                    "reason" => "Failed to add post."
                })
    }
}

#[get("/<id>")]
fn get_post(id: i32) -> JSON<Message<Post>>  {
    let post = get_post_by_id(id);
    match post {
        Ok(p) => {
            let message = Message{
                model: Some(p),
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
