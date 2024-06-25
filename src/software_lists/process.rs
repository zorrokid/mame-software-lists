use diesel::Connection;
use diesel::RunQueryDsl;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::database::software_lists::*;
use crate::xml_parser::parse_file;
use crate::schema::{software_lists, machines, roms};

pub fn process_from_datafile(path: String){
    let connection = &mut establish_connection();
    match parse_file(&path){
        Ok(datafile) => {
            match software_list_exists(connection, datafile.header.name.clone(), datafile.header.version.clone()) {
                true => {
                    println!("Software list already exists");
                    std::process::exit(1);
                },

                false => {
                    match connection.transaction(|connection|{

                        let inserted_software_list_id: i32 = diesel::insert_into(software_lists::table)
                            .values((
                                software_lists::name.eq(datafile.header.name),
                                software_lists::description.eq(datafile.header.description),
                                software_lists::version.eq(datafile.header.version),
                                software_lists::author.eq(datafile.header.author)

                            ))
                            .returning(software_lists::id)
                            .get_result(connection)?;
                    
                        for machine in datafile.machines {
                           let inserted_machine_id: i32 = diesel::insert_into(machines::table)
                                .values((
                                    machines::description.eq(machine.description.clone()),
                                    machines::year.eq(machine.year.map(|x| x as i32)),
                                    machines::publisher.eq(machine.publisher),
                                    machines::software_list_id.eq(inserted_software_list_id)
                                ))
                                .returning(machines::id)
                                .get_result(connection)?;

                            for rom in machine.rom {
                               let inserted_rom_id: i32 = diesel::insert_into(crate::schema::roms::table)
                                    .values((
                                        roms::name.eq(rom.name),
                                        roms::size.eq(rom.size as i32),
                                        roms::crc.eq(rom.crc),
                                        roms::sha1.eq(rom.sha1)
                                    ))
                                    .returning(roms::id)
                                    .get_result(connection)?;

                                diesel::insert_into(crate::schema::machines_roms::table)
                                    .values((
                                        crate::schema::machines_roms::machine_id.eq(inserted_machine_id),
                                        crate::schema::machines_roms::rom_id.eq(inserted_rom_id)
                                    ))
                                    .execute(connection)?;
                            }
                        }
                        diesel::result::QueryResult::Ok(())
                    }) {
                       Ok(_) => println!("Software list stored successfully!"),
                       Err(_) => println!("Error storing software list: {}", path) 
                    }
                }
            }
        },
        Err(e) => println!("Error parsing file: {}", e),
    }
}