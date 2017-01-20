extern crate rocket;
extern crate serde_json;

use rocket_contrib::JSON;

use models::posts::{Post, NewPost, get_post_by_id};

#[derive(Serialize)]
pub struct ModelMessage<T> {
    model: Option<T>,
    error: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    status: String,
    reason: Option<String>,
}


#[post("/", data="<new_post>", format="application/json")]
pub fn new_post(new_post: JSON<NewPost>) -> JSON<ResponseMessage> {
    if new_post.insert() {
        JSON(
            ResponseMessage{
                status: "Ok".to_string(),
                reason: None,
            }
        )
    } else {
        JSON(
            ResponseMessage{
                status: "Error".to_string(),
                reason: Some("Failed to add user".to_string()),
            }
        )

    }
}

#[get("/<id>")]
fn get_post(id: i32) -> JSON<ModelMessage<Post>>  {
    let post = get_post_by_id(id);
    match post {
        Ok(p) => {
            JSON(ModelMessage{
                model: Some(p),
                error: "".to_string(),
            })
        }
        Err(e) => {
            JSON(ModelMessage{
                model: None,
                error: format!("{}", e),
            })
        }
    }
}

#[delete("/<id>")]
fn delete_post(id: i32) -> JSON<ResponseMessage> {
    let post = get_post_by_id(id);
    match post {
        Ok(p) => {
            if p.delete() {
                JSON(
                    ResponseMessage{
                        status: "Ok".to_string(),
                        reason: None,
                    }
                )
            } else {
                JSON(
                    ResponseMessage{
                        status: "Error".to_string(),
                        reason: Some("Failed to delete post".to_string()),
                    }
                )
            }
        }
        Err(e) => {
            JSON(
                ResponseMessage{
                    status: "Error".to_string(),
                    reason: Some("Failed to find post".to_string()),
                }
            )
        }
    }
}
