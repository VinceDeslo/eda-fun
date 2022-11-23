use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: String,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub bio: &'a str,
}