use diesel::prelude::*;
use crate::schema::software_lists;
use crate::models;


pub fn db_get_software_lists(conn: &mut SqliteConnection) -> Result<Vec<models::SoftwareList>, diesel::result::Error> {
    software_lists::table.load::<models::SoftwareList>(conn)
}

pub fn db_get_software_list(conn: &mut SqliteConnection, input_id: i32) -> Result<models::SoftwareList, diesel::result::Error> {
    use crate::schema::software_lists::dsl::*;
    software_lists.filter(id.eq(input_id))
        .first(conn)
}


pub fn software_list_exists(conn: &mut SqliteConnection, sofware_list_name: String, software_list_version: String) -> bool {
    use crate::schema::software_lists::dsl::*;
    let results = software_lists
        .filter(name.eq(sofware_list_name))
        .filter(version.eq(software_list_version))
        .limit(1)
        .load::<models::SoftwareList>(conn)
        .expect("Error loading software list");

    results.len() > 0
}

pub fn db_get_software_lists_for_system(conn: &mut SqliteConnection, input_system_id: i32) -> Result<Vec<models::SoftwareList>, diesel::result::Error> {
    use crate::schema::software_lists::dsl::*;
    software_lists.filter(system_id.eq(input_system_id))
        .load::<models::SoftwareList>(conn)
}

pub fn delete_software_list(conn: &mut SqliteConnection, software_list_name: String, software_list_version: String) -> Result<(), diesel::result::Error> {
    use crate::schema::software_lists::dsl::*;
    diesel::delete(software_lists.filter(name.eq(software_list_name)).filter(version.eq(software_list_version)))
        .execute(conn)
        .map(|_| ())
}

