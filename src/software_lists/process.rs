use diesel::prelude::*;
use diesel::Connection;
use diesel::RunQueryDsl;

use crate::database::software_lists::*;
use crate::database::systems::db_get_system;
use crate::schema::{machines, roms, software_lists};
use crate::xml_parser::parse_file;

pub fn process_from_datafile(connection: &mut SqliteConnection, path: String) {
    match parse_file(&path) {
        Ok(datafile) => {
            match software_list_exists(
                connection,
                datafile.header.name.clone(),
                datafile.header.version.clone(),
            ) {
                true => {
                    println!("Software list already exists");
                    std::process::exit(1);
                }

                false => {
                    match connection.transaction(|connection| {
                        let system_name = datafile.header.name;
                        let system = db_get_system(connection, system_name.clone())?;
                        let system_id = match system {
                            Some(system) => system.id,
                            None => {
                                let inserted_system_id: i32 =
                                    diesel::insert_into(crate::schema::systems::table)
                                        .values(
                                            crate::schema::systems::name.eq(system_name.clone()),
                                        )
                                        .returning(crate::schema::systems::id)
                                        .get_result(connection)?;
                                inserted_system_id
                            }
                        };

                        let inserted_software_list_id: i32 =
                            diesel::insert_into(software_lists::table)
                                .values((
                                    software_lists::name.eq(system_name),
                                    software_lists::description.eq(datafile.header.description),
                                    software_lists::version.eq(datafile.header.version),
                                    software_lists::author.eq(datafile.header.author),
                                    software_lists::system_id.eq(system_id),
                                ))
                                .returning(software_lists::id)
                                .get_result(connection)?;

                        for machine in datafile.machines {
                            let inserted_machine_id: i32 = diesel::insert_into(machines::table)
                                .values((
                                    machines::description.eq(machine.description.clone()),
                                    machines::year.eq(machine.year.map(|x| x as i32)),
                                    machines::publisher.eq(machine.publisher),
                                    machines::software_list_id.eq(inserted_software_list_id),
                                    machines::name.eq(machine.name),
                                ))
                                .returning(machines::id)
                                .get_result(connection)?;

                            for rom in machine.rom {
                                let inserted_rom_id: i32 =
                                    diesel::insert_into(crate::schema::roms::table)
                                        .values((
                                            roms::name.eq(rom.name),
                                            roms::size.eq(rom.size as i32),
                                            roms::crc.eq(rom.crc),
                                            roms::sha1.eq(rom.sha1),
                                        ))
                                        .returning(roms::id)
                                        .get_result(connection)?;

                                diesel::insert_into(crate::schema::machines_roms::table)
                                    .values((
                                        crate::schema::machines_roms::machine_id
                                            .eq(inserted_machine_id),
                                        crate::schema::machines_roms::rom_id.eq(inserted_rom_id),
                                    ))
                                    .execute(connection)?;
                            }
                        }
                        diesel::result::QueryResult::Ok(())
                    }) {
                        Ok(_) => println!("Software list stored successfully!"),
                        Err(_) => println!("Error storing software list: {}", path),
                    }
                }
            }
        }
        Err(e) => println!("Error parsing file: {}", e),
    }
}
