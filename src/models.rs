use diesel::prelude::*;
use crate::schema::{software_list, machine, rom};
use crate::software_list_models;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = software_list)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SoftwareList {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

impl From<software_list_models::Header> for SoftwareList {
    fn from(header: software_list_models::Header) -> Self {
        SoftwareList {
            id: None,
            name: header.name,
            description: header.description,
            version: header.version,
            author: header.author,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = machine)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Machine {
    pub id: Option<i32>,
    pub description: String,
    pub year: i32,
    pub publisher: String,
    pub software_list_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = rom)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rom {
    pub id: Option<i32>,
    pub name: String,
    pub size: i32,
    pub crc: String,
    pub sha1: String,
    pub machine_id: i32,
}