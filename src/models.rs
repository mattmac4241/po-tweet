use schema::users;

use diesel;

use database::db;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn insert(&self) -> bool {
        diesel::insert(self).into(users::table).execute(&db()).is_ok()
    }
}
