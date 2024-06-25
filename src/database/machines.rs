use diesel::prelude::*;
use crate::models::{Machine, SoftwareList};

pub fn db_get_machines(conn: &mut SqliteConnection, software_list: &SoftwareList) -> Result<Vec<Machine>, diesel::result::Error> {
    Machine::belonging_to(&software_list)
        .select(Machine::as_select())
        .load(conn)
}