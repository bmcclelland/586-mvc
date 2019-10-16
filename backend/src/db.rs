use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn open() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set"));

    let db_con = SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    diesel::sql_query("PRAGMA foreign_keys=ON")
        .execute(&db_con)
        .unwrap();

    db_con
}
