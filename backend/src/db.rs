use common::*;
use common::schema::*;
use failure::Error;
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


macro_rules! last_id (
    ($table: ident :: $field: ident, $db_con: expr) => {
        $table::table.select($table::$field)
            .order($table::$field.desc())
            .first($db_con)
    }
);

pub trait Insert {
    type ID;
    fn insert(&self, _: &SqliteConnection) -> Result<Self::ID, Error>;
}

impl Insert for Project {
    type ID = ProjectID;

    fn insert(&self, db: &SqliteConnection) -> Result<Self::ID, Error> {
       self.insert_into(projects::table).execute(db)?;
       let id = last_id!(projects::id, db)?;
       Ok(id)
    }
}

impl Insert for Task {
    type ID = TaskID;

    fn insert(&self, db: &SqliteConnection) -> Result<Self::ID, Error> {
       self.insert_into(tasks::table).execute(db)?;
       let id = last_id!(tasks::id, db)?;
       Ok(id)
    }
}

impl Insert for Worker {
    type ID = WorkerID;

    fn insert(&self, db: &SqliteConnection) -> Result<Self::ID, Error> {
       self.insert_into(workers::table).execute(db)?;
       let id = last_id!(workers::id, db)?;
       Ok(id)
    }
}
