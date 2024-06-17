use diesel::prelude::*;
use crate::{schema::software_list, software_list_models::{DataFile, Header, Machine, Rom}};
use crate::models;

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

pub fn delete_software_list(conn: &mut SqliteConnection, name: String, version: String) -> Result<(), diesel::result::Error> {
    use crate::schema::software_list::dsl::*;
    diesel::delete(software_list.filter(name.eq(name)).filter(version.eq(version)))
        .execute(conn)
        .map(|_| ())
}

