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

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rom {
    pub id: i32,
    pub name: String,
    pub size: i32,
    pub crc: String,
    pub sha1: String,
    pub have: bool,
}


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