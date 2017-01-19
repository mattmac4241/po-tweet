use schema::users;
use schema::users::dsl::{users as all_users};

use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use bcrypt::{DEFAULT_COST, hash};

use database::db;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name="users"]
#[has_many(posts)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[belongs_to(User)]
#[table_name="posts"]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

impl NewUser {
    pub fn insert(&mut self) -> bool {
        match hash(&self.password, DEFAULT_COST) {
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
