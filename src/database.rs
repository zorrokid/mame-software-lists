use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::{schema::software_list, software_list_models::{DataFile, Header, Machine, Rom}};
use crate::models;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_software_list(conn: &mut SqliteConnection, software_list: models::SoftwareList) -> Result<(), diesel::result::Error> {
    diesel::insert_into(software_list::table)
        .values(&software_list)
        .execute(conn)
        .map(|_| ())
}

pub fn get_software_lists(conn: &mut SqliteConnection) -> Result<Vec<models::SoftwareList>, diesel::result::Error> {
    software_list::table.load::<models::SoftwareList>(conn)
}

pub fn software_list_exists(conn: &mut SqliteConnection, name: String, version: String) -> bool {
    use crate::schema::software_list::dsl::*;
    let results = software_list
        .filter(name.eq(name))
        .filter(version.eq(version))
        .limit(1)
        .load::<models::SoftwareList>(conn)
        .expect("Error loading software list");

    results.len() > 0
}



/*use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::{HeaderMachine, Header, Machine, Rom, DataFile};

pub fn get_connection() -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish("mame_software_lists.db")
}


pub fn insert_datafile(conn: &SqliteConnection, datafile: &DataFile) -> Result<(), rusqlite::Error> {
    diesel::insert_into(Header::table)
        .values(&datafile.header)
        .execute(conn)
        .map(|_| ());

    let header_id = Header::table;
    for machine in &datafile.machines {
        diesel::insert_into(Machine::table)
            .values(machine)
            .execute(conn)
            .map(|_| ())?;

        let machine_id = Machine::table
            .select(Machine::id)
            .filter(Machine::name.eq(&machine.name))
            .first::<i32>(conn)?;

        diesel::insert_into(HeaderMachine::table)
            .values((
                HeaderMachine::header_id.eq(header_id),
                HeaderMachine::machine_id.eq(machine_id),
            ))
            .execute(conn)
            .map(|_| ())?;


        for rom in &machine.rom {
            diesel::insert_into(Rom::table)
                .values(rom)
                .execute(conn)
                .map(|_| ())?;

            let rom_id = Rom::table
                .select(Rom::id)
                .filter(Rom::name.eq(&rom.name))
                .first::<i32>(conn)?;

            diesel::insert_into(machine_rom::table)
                .values((
                    machine_rom::machine_id.eq(machine_id),
                    machine_rom::rom_id.eq(rom_id),
                ))
                .execute(conn)
                .map(|_| ())?;
        }
    }

    Ok(())
}


pub fn get_headers(conn: &SqliteConnection) -> Result<Vec<Header>, rusqlite::Error> {
    Header::table.load::<Header>(conn).expect("Error loading headers")
}
    */