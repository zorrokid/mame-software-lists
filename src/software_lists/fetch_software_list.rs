use std::collections::HashMap;

use diesel::SqliteConnection;

use crate::models;
use crate::database::software_lists::db_get_software_list;
use crate::database::machines::db_get_machines;
use crate::database::roms::db_get_roms;

pub fn fetch_software_list_roms(connection: &mut SqliteConnection, id: i32) -> Result<HashMap<String, models::Rom>, diesel::result::Error> {

    let software_list = db_get_software_list(connection, id)?;
    println!("{:?}", software_list);
    let machines = db_get_machines(connection, &software_list)?;

    let mut roms: HashMap<String, models::Rom> = HashMap::new();

    for machine in machines {
        let machine_roms = db_get_roms(connection, &machine)?;
        for rom in machine_roms {
            roms.insert(rom.sha1.clone(), rom);
        }
    }

    Ok(roms)
}