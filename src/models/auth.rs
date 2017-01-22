use schema::{auth_tokens};

use chrono::NaiveDateTime;
use chrono::offset::utc::UTC;
use chrono::Duration;

use diesel;
use diesel::result::Error;
use diesel::prelude::*;
use database::db;

use jwt::{decode, encode, Header, Algorithm, TokenData};
use jwt::errors;

use std::env;
use std::ops::Add;

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[belongs_to(users)]
#[table_name="auth_tokens"]
pub struct Token {
    id: i32,
    key: String,
    expires_at: NaiveDateTime,
    user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="auth_tokens"]
pub struct NewToken {
    key: String,
    expires_at: NaiveDateTime,
    user_id: i32,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct JWT {
    pub user_id: i32,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        self.expires_at.timestamp() <= UTC::now().timestamp()
    }
}

impl NewToken {
    pub fn insert(&self) -> bool {
        diesel::insert(self).into(auth_tokens::table).execute(&db()).is_ok()
    }
}

pub fn get_token_from_key(_key: String) -> Result<Token, Error> {
    auth_tokens::table.filter(auth_tokens::key.eq(_key)).first(&db())
}

pub fn create_token(user_id: i32) -> Option<NewToken> {
    let key = generate_token(user_id);
    match key {
        Ok(k) => {
            let token = NewToken {
                key: k,
                user_id: user_id,
                expires_at: sixty_days_from_now(),
            };
            if token.insert() {
                Some(token)
            } else {
                None
            }
        }
        Err(_) => None
    }
}

//generate_token, creates the jwt key
pub fn generate_token(_user_id: i32) -> Result<String, errors::Error> {
    let key = env::var("SECRET_KEY");
    let jwt = JWT{
        user_id: _user_id,
    };
    encode(Header::default(), &jwt, key.unwrap().as_ref())
}

pub fn decode_token(token: String) -> Option<JWT> {
    let key = env::var("SECRET_KEY");
    match decode::<JWT>(&token, key.unwrap().as_ref(), Algorithm::HS256) {
        Ok(c) => Some(c.claims),
        Err(_) => None
    }
}

fn sixty_days_from_now() -> NaiveDateTime {
    UTC::now().naive_utc().add(Duration::days(60))
}
