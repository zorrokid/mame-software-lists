use diesel::prelude::*;
use crate::models::{Machine, SoftwareList};

pub fn db_get_machines(conn: &mut SqliteConnection, software_list: &SoftwareList) -> Result<Vec<Machine>, diesel::result::Error> {
    Machine::belonging_to(&software_list)
        .select(Machine::as_select())
        .load(conn)
}

pub fn db_get_machine(conn: &mut SqliteConnection, input_id: i32) -> Result<Machine, diesel::result::Error> {
    use crate::schema::machines::dsl::*;
    machines.filter(id.eq(input_id))
        .first(conn)
}

pub fn db_get_machines_for_software_list(conn: &mut SqliteConnection, input_software_list_id: i32) -> Result<Vec<Machine>, diesel::result::Error> {
    use crate::schema::machines::dsl::*;
    machines.filter(software_list_id.eq(input_software_list_id))
        .load(conn)
}