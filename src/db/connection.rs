use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    SqliteConnection::establish(&database_url).unwrap()}
