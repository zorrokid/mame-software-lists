use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod machines;
pub mod roms;
pub mod software_lists;
pub mod systems;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
