use diesel::prelude::*;
use crate::schema::{machines, roms, software_lists,  machines_roms};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SoftwareList {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}


/*impl From<software_list_models::Header> for NewSoftwareList {
    fn from(header: software_list_models::Header) -> Self {
        NewSoftwareList {
            name: header.name,
            description: header.description,
            version: header.version,
            author: header.author,
        }
    }
}*/


#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(SoftwareList))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Machine {
    pub id: i32,
    pub description: String,
    pub year: Option<i32>,
    pub publisher: String,
    pub software_list_id: i32,
}

/*impl From<software_list_models::Machine> for Machine {
    fn from(machine: software_list_models::Machine) -> Self {
        Machine {
            id: None,
            description: machine.description,
            year: machine.year.map(|x| x as i32),
            publisher: machine.publisher,
            software_list_id: 0,
        }
    }
}*/

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rom {
    pub id: i32,
    pub name: String,
    pub size: i32,
    pub crc: String,
    pub sha1: String,
}

/*impl From<software_list_models::Rom> for Rom {
    fn from(rom: software_list_models::Rom) -> Self {
        Rom {
            id: None,
            name: rom.name,
            size: rom.size as i32,
            crc: rom.crc,
            sha1: rom.sha1,
        }
    }
}*/

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Machine))]
#[diesel(belongs_to(Rom))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(machine_id, rom_id))]
#[diesel(table_name = machines_roms)]
pub struct MachineRom {
    pub machine_id: i32,
    pub rom_id: i32,
}