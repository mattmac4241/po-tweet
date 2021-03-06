use rocket_contrib::JSON;

use models::users::{NewUser, get_by_username, LoginUser};
use models::auth::{create_token, NewToken};

use routes::routes::*;


#[post("/register", data="<new_user>", format="application/json")]
pub fn register(new_user: JSON<NewUser>) -> JSON<ModelMessage<NewToken>> {
    let mut user = new_user;
    match user.insert() {
        None => {
            create_model_message(None, Some("Failed to add user.".to_string()))
        }
        Some(u) => {
            match create_token(u.id) {
                Some(token) => {
                    create_model_message(Some(token), None)

                }
                None => {
                    create_model_message(None, Some("Failed to generate token".to_string()))
                }
            }
        }
    }
}


#[post("/login", data="<login_user>", format="application/json")]
pub fn login(login_user: JSON<LoginUser>) -> JSON<ModelMessage<NewToken>> {
    match login_user.validate_user() {
        None => {
            create_model_message(None, Some("Failed to login.".to_string()))
        }
        Some(u) => {
            match create_token(u.id) {
                Some(token) => {
                    create_model_message(Some(token), None)
                }
                None => {
                    create_model_message(None, Some("Failed to generate token".to_string()))
                }
            }
        }
    }
}
