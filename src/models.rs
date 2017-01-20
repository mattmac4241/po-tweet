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
    user_id: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    title: String,
    body: String,
    user_id: i32,
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

impl NewPost {
    pub fn insert(&self) -> bool {
        diesel::insert(self).into(posts::table).execute(&db()).is_ok()
    }
}

impl Post {
    pub fn delete(&self) -> bool {
        diesel::delete(posts::table.filter(posts::id.eq(self.id))).execute(&db()).is_ok()
    }
}

pub fn get_user_by_id(_id: i32) -> Result<User, Error> {
    users::table.find(_id).get_result::<User>(&db())
}

pub fn get_user_by_username(name: String) -> Result<User, Error> {
    users::table.filter(users::username.eq(name)).first(&db())
}

pub fn get_post_by_id(_id: i32) -> Result<Post, Error> {
    posts::table.find(_id).get_result::<Post>(&db())
}
