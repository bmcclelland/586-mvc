use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn open_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db_con = SqliteConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    diesel::sql_query("PRAGMA foreign_keys=ON")
        .execute(&db_con)
        .unwrap();

    return db_con;
}
