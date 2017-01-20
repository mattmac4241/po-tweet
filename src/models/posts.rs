use schema::{posts};

use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use database::db;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[belongs_to(users)]
#[table_name="posts"]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    title: String,
    body: String,
    user_id: i32,
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

pub fn get_post_by_id(_id: i32) -> Result<Post, Error> {
    posts::table.find(_id).get_result::<Post>(&db())
}
