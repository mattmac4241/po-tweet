use schema::{users, posts};

use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use bcrypt::{DEFAULT_COST, hash};

use database::db;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name="users"]
#[has_many(posts)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

impl NewUser {

    pub fn insert(&mut self) -> Option<User> {
        match hash(&self.password, DEFAULT_COST) {
            Ok(h) => { self.password = format!("{}",h) }
            Err(_) => return None,
        };
        match diesel::insert(self).into(users::table).get_result::<User>(&db()) {
            Ok(u) => Some(u),
            Err(_) => None
        }
    }
}

pub fn get_by_id(_id: i32) -> Result<User, Error> {
    users::table.find(_id).get_result::<User>(&db())
}

pub fn get_by_username(name: String) -> Result<User, Error> {
    users::table.filter(users::username.eq(name)).first(&db())
}
