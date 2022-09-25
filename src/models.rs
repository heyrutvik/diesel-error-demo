use super::schema::users;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub birth_date: NaiveDate,
}
