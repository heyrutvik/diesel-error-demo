use crate::models::*;
use crate::schema::users::dsl::users;
use crate::schema::users::{birth_date, id, name};
use diesel::insert_into;
use diesel::prelude::*;

pub fn create_user(new_user: NewUser, conn: &mut PgConnection) -> QueryResult<User> {
    insert_into(users).values(new_user).get_result(conn)
}

pub fn fetch_user(unique_name: String, conn: &mut PgConnection) -> QueryResult<User> {
    users.filter(name.eq(unique_name)).first::<User>(conn)
}
