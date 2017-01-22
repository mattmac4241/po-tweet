use rocket_contrib::JSON;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use models::auth::{Token, get_token_from_key};

#[derive(Serialize)]
pub struct ModelMessage<T> {
    model: Option<T>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    status: String,
    reason: Option<String>,
}

pub fn create_model_message<T>(_model: Option<T>, _error: Option<String>) -> JSON<ModelMessage<T>> {
    JSON(ModelMessage{
            model: _model,
            error: _error
        })
}

pub fn create_response_message(_status: String, _reason: Option<String>) -> JSON<ResponseMessage> {
    JSON(ResponseMessage{
            status: _status,
            reason: _reason
        })
}

pub struct APIKey(pub String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: String) -> bool {
    match get_token_from_key(key) {
        Ok(token) => !token.is_expired(),
        Err(_) => false
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for APIKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<APIKey, ()> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if !is_valid(keys[0].to_string()) {
            return Outcome::Forward(());
        }

        return Outcome::Success(APIKey(key.to_string()));
    }
}
