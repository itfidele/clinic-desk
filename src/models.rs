use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}
