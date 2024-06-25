use diesel::SqliteConnection;

use crate::models;
use crate::database::software_lists::db_get_software_list;
use crate::database::machines::db_get_machines;
use crate::database::roms::db_get_roms;

pub fn fetch_software_list(connection: &mut SqliteConnection, id: i32) -> Result<models::SoftwareList, diesel::result::Error> {

    let software_list = db_get_software_list(connection, id)?;
    //println!("{:?}", software_list);

    let machines = db_get_machines(connection, &software_list)?;
    for machine in machines {
        println!("{:?}", machine);
        let rom = db_get_roms(connection, &machine)?;
        for r in rom {
            println!("{:?}", r);
        }
    }

    Ok(software_list)
}