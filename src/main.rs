use crate::lib::establish_connection;
use crate::models::NewUser;
use chrono::NaiveDate;
use diesel::result::Error::DatabaseError;
use diesel::{PgConnection, QueryResult};

mod generic;
mod lib;
mod models;
mod schema;

fn main() {
    let name = "name_1".to_owned();
    let birth_date = NaiveDate::from_ymd(2022, 9, 25);
    let new_user = NewUser { name, birth_date };

    // THIS SCENARIO IS ACTUALLY HAPPENING WHILE USING BB8 ASYNC CONNECTION POOL

    // assume this was created by pool manager and provide it to
    // a caller till it can make sure that connection is valid
    let conn = &mut establish_connection();

    // first request comes in, use `conn` to fulfill the operation
    create_or_fetch(new_user.clone(), conn);

    // second request comes in, pool manager provides the same
    // connection because it's valid. but while doing the 'fetch'
    // query it gets 'another command is already in progress'.
    create_or_fetch(new_user.clone(), conn);

    // on third request, pool manager will find out that the connection
    // is not valid anymore, drops it and creates new connection to
    // fulfill the operation
    // let conn = &mut establish_connection();
    // create_or_fetch(new_user.clone(), conn);
}

fn create_or_fetch(new_user: NewUser, conn: &mut PgConnection) {
    let create_user_result = generic::create_user(new_user.clone(), conn);
    match create_user_result {
        Ok(user) => println!("1.Result: {:?}", user),
        Err(DatabaseError(UniqueViolation, _)) => {
            let fetch_user_result = generic::fetch_user(new_user.name, conn);
            match fetch_user_result {
                Ok(user) => println!("2.Result: {:?}", user),
                Err(e) => println!("2.Error: {}", e),
            }
        }
        Err(e) => println!("1.Error: {}", e),
    }
}
