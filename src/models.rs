use schema::users;
use schema::users::dsl::{users as all_users};

use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use bcrypt::{DEFAULT_COST, hash, verify};

use database::db;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name="users"]
pub struct User {
    id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn insert(&mut self) -> bool {
        let hashed = match hash(&self.password, DEFAULT_COST) {
            Ok(h) => {
                self.password = format!("{}",h);
            }
            Err(_) => return false
        };

        diesel::insert(self).into(users::table).execute(&db()).is_ok()
    }
}

pub fn get_user_by_id(user_id: i32) -> Result<User, Error> {
    let user = all_users.find(user_id).get_result::<User>(&db());
    user
}
