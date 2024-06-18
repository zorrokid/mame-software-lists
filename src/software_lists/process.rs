use diesel::Connection;
use diesel::RunQueryDsl;

use crate::database::establish_connection;
use crate::database::software_lists::*;
use crate::xml_parser::parse_file;
use crate::schema::{software_list, machine};
use crate::models;

pub fn process_from_datafile(path: String){
    let connection = &mut establish_connection();
    match parse_file(&path){
        Ok(datafile) => {
            // TODO: move to database module and use transactions
            // https://stackoverflow.com/questions/75939019/transactions-in-rust-diesel
            let software_list: models::SoftwareList = datafile.header.into();
            match software_list_exists(connection, software_list.name.clone(), software_list.version.clone()) {
                true => {
                    println!("Software list already exists");
                    std::process::exit(1);
                },

                false => {
                    match connection.transaction(|connection|{

                        let inserted_software_list_id: Option<i32> = diesel::insert_into(software_list::table)
                            .values(&software_list)
                            .returning(software_list::id)
                            .get_result(connection)?;
                    
                        for machine in datafile.machines {
                            let mach: models::Machine = models::Machine{
                                id: None,
                                description: machine.description,
                                year: machine.year.map(|x| x as i32),
                                publisher: machine.publisher,
                                software_list_id: inserted_software_list_id.unwrap()
                            };
                            diesel::insert_into(machine::table)
                                .values(&mach)
                                .execute(connection)?;
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