use crate::models::{Machine, Rom, SoftwareList, System};
use crate::schema::{machines, machines_roms, roms, software_lists};
use crate::software_list_models::DataFile;
use diesel::prelude::*;
use diesel::SqliteConnection;

pub struct DataAccessError {
    pub message: String,
}

pub trait DataAccessTrait {
    fn fetch_software_list_roms(
        &mut self,
        software_list: &SoftwareList,
    ) -> Result<Vec<Rom>, DataAccessError>;
    fn set_matched_roms(
        &mut self,
        software_list: &SoftwareList,
        checksums: &Vec<String>,
    ) -> Result<usize, DataAccessError>;
    fn get_software_lists_for_system(
        &mut self,
        system_id: i32,
    ) -> Result<Vec<SoftwareList>, DataAccessError>;
    fn get_machines_for_software_list(
        &mut self,
        software_list_id: i32,
    ) -> Result<Vec<Machine>, DataAccessError>;
    fn get_roms_for_machine(&mut self, machine: &Machine) -> Result<Vec<Rom>, DataAccessError>;
    fn get_system(&mut self, system_name: String) -> Result<Option<System>, DataAccessError>;
    fn get_systems(&mut self) -> Result<Vec<System>, DataAccessError>;
    fn get_software_lists(&mut self) -> Result<Vec<SoftwareList>, DataAccessError>;
    fn software_list_exists(
        &mut self,
        sofware_list_name: String,
        software_list_version: String,
    ) -> bool;
    fn process_software_list(&mut self, data_file: &DataFile) -> Result<(), DataAccessError>;
}

pub struct DataAccessProvider {
    connection: SqliteConnection,
}

impl<'a> DataAccessProvider {
    pub fn new() -> Self {
        let connection = crate::database::establish_connection();
        Self { connection }
    }
}

impl<'a> DataAccessTrait for DataAccessProvider {
    fn fetch_software_list_roms(
        &mut self,
        software_list: &SoftwareList,
    ) -> Result<Vec<Rom>, DataAccessError> {
        let roms = software_lists::table
            .inner_join(machines::table.on(machines::software_list_id.eq(software_lists::id)))
            .inner_join(machines_roms::table.on(machines_roms::machine_id.eq(machines::id)))
            .inner_join(roms::table.on(roms::id.eq(machines_roms::rom_id)))
            .filter(software_lists::id.eq(software_list.id))
            .select(roms::all_columns)
            .load::<Rom>(&mut self.connection)
            .map_err(|e| DataAccessError {
                message: format!("Error fetching roms: {}", e),
            })?;
        Ok(roms)
    }

    fn set_matched_roms(
        &mut self,
        software_list: &SoftwareList,
        checksums: &Vec<String>,
    ) -> Result<usize, DataAccessError> {
        let software_list_roms =
            self.fetch_software_list_roms(software_list)
                .map_err(|e| DataAccessError {
                    message: format!("Error fetching software list roms: {}", e.message),
                })?;

        let software_list_roms_updated = software_list_roms
            .into_iter()
            .map(|x| {
                let mut clone = x.clone();
                clone.available = Some(checksums.contains(&x.sha1));
                clone
            })
            .collect();

        crate::database::roms::update(&mut self.connection, &software_list_roms_updated).map_err(
            |e| DataAccessError {
                message: format!("Error setting matched roms: {}", e),
            },
        )?;

        let available_roms_count = software_list_roms_updated
            .iter()
            .filter(|x| x.available.unwrap_or(false))
            .count();

        Ok(available_roms_count)
    }

    fn get_software_lists_for_system(
        &mut self,
        system_id: i32,
    ) -> Result<Vec<SoftwareList>, DataAccessError> {
        crate::database::software_lists::db_get_software_lists_for_system(
            &mut self.connection,
            system_id,
        )
        .map_err(|e| DataAccessError {
            message: format!("Error fetching software lists for system: {}", e),
        })
    }

    fn get_machines_for_software_list(
        &mut self,
        software_list_id: i32,
    ) -> Result<Vec<Machine>, DataAccessError> {
        crate::database::machines::db_get_machines_for_software_list(
            &mut self.connection,
            software_list_id,
        )
        .map_err(|e| DataAccessError {
            message: format!("Error fetching machines for software list: {}", e),
        })
    }

    fn get_roms_for_machine(&mut self, machine: &Machine) -> Result<Vec<Rom>, DataAccessError> {
        crate::database::roms::db_get_roms(&mut self.connection, machine).map_err(|e| {
            DataAccessError {
                message: format!("Error fetching roms for machine: {}", e),
            }
        })
    }

    fn get_system(&mut self, system_name: String) -> Result<Option<System>, DataAccessError> {
        crate::database::systems::db_get_system(&mut self.connection, system_name).map_err(|e| {
            DataAccessError {
                message: format!("Error fetching system: {}", e),
            }
        })
    }

    fn get_systems(&mut self) -> Result<Vec<System>, DataAccessError> {
        crate::database::systems::db_get_systems(&mut self.connection).map_err(|e| {
            DataAccessError {
                message: format!("Error fetching systems: {}", e),
            }
        })
    }

    fn get_software_lists(&mut self) -> Result<Vec<SoftwareList>, DataAccessError> {
        crate::database::software_lists::db_get_software_lists(&mut self.connection).map_err(|e| {
            DataAccessError {
                message: format!("Error fetching software lists: {}", e),
            }
        })
    }

    fn software_list_exists(
        &mut self,
        sofware_list_name: String,
        software_list_version: String,
    ) -> bool {
        crate::database::software_lists::software_list_exists(
            &mut self.connection,
            sofware_list_name,
            software_list_version,
        )
    }

    // TODO maybe move the transaction to database module
    fn process_software_list(&mut self, datafile: &DataFile) -> Result<(), DataAccessError> {
        let system_name = datafile.header.name.clone();
        let system = self.get_system(system_name.clone())?;

        match self.connection.transaction(|connection| {
            let system_id = match system {
                Some(system) => system.id,
                None => {
                    let inserted_system_id: i32 =
                        diesel::insert_into(crate::schema::systems::table)
                            .values(crate::schema::systems::name.eq(system_name.clone()))
                            .returning(crate::schema::systems::id)
                            .get_result(connection)?;
                    inserted_system_id
                }
            };

            let inserted_software_list_id: i32 = diesel::insert_into(software_lists::table)
                .values((
                    software_lists::name.eq(system_name),
                    software_lists::description.eq(datafile.header.description.clone()),
                    software_lists::version.eq(datafile.header.version.clone()),
                    software_lists::author.eq(datafile.header.author.clone()),
                    software_lists::system_id.eq(system_id),
                ))
                .returning(software_lists::id)
                .get_result(connection)?;

            for machine in &datafile.machines {
                let inserted_machine_id: i32 = diesel::insert_into(machines::table)
                    .values((
                        machines::description.eq(machine.description.clone()),
                        machines::year.eq(machine.year.map(|x| x as i32)),
                        machines::publisher.eq(machine.publisher.clone()),
                        machines::software_list_id.eq(inserted_software_list_id),
                        machines::name.eq(machine.name.clone()),
                    ))
                    .returning(machines::id)
                    .get_result(connection)?;

                for rom in &machine.rom {
                    let inserted_rom_id: i32 = diesel::insert_into(crate::schema::roms::table)
                        .values((
                            roms::name.eq(rom.name.clone()),
                            roms::size.eq(rom.size as i32),
                            roms::crc.eq(rom.crc.clone()),
                            roms::sha1.eq(rom.sha1.clone()),
                        ))
                        .returning(roms::id)
                        .get_result(connection)?;

                    diesel::insert_into(crate::schema::machines_roms::table)
                        .values((
                            crate::schema::machines_roms::machine_id.eq(inserted_machine_id),
                            crate::schema::machines_roms::rom_id.eq(inserted_rom_id),
                        ))
                        .execute(connection)?;
                }
            }
            diesel::result::QueryResult::Ok(())
        }) {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError {
                message: "Error storing software list".to_string(),
            }),
        }
    }
}
